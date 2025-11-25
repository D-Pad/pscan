use std::{
    collections::VecDeque, 
    env, 
    fmt, 
    fs, 
    io::{BufReader, BufRead}, 
    path::Path,
    cmp::min
};

use crate::arguments::parser::{ParsedArgs, HELP_TEXT};
pub mod arguments;


pub enum PscanError {
    FileRead,
    Argument,
    InputError
}

impl fmt::Display for PscanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &str = match &self {
            Self::FileRead => "FileRead", 
            Self::Argument => "Argument",
            Self::InputError => "InputError"
        }; 
        write!(f, "{}", msg)
    }
}

pub struct ErrorResponse {
    pub error_msg: String,
    pub error_type: PscanError
}

impl ErrorResponse {
    fn new(error_msg: String, error_type: PscanError) -> Self {
        ErrorResponse { error_msg, error_type }
    }
}

// This is all you need for Display
impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[1;31m{}: {}\x1b[0m", self.error_type, self.error_msg)
    }
}

pub fn error_handler(error_response: ErrorResponse) -> i32 {
    
    let mut err_msg = String::from("\x1b[1;31m");
    let mut exit_code = 1; 

    match error_response.error_type {
        PscanError::FileRead => {
            err_msg.push_str("FileReadError: ");
        },
        PscanError::Argument => {
            err_msg.push_str("BadArgumentError: ");
            exit_code = 2;
        },
        PscanError::InputError => {
            err_msg.push_str("InputError: ");
            exit_code = 2;
        }
    }
    
    err_msg.push_str(&format!("{}", error_response.error_msg));
    println!("{}\x1b[0m", err_msg);
    exit_code

}


fn process_paths_from_args(
    parsed_args: &ParsedArgs
) -> Result<usize, ErrorResponse> {

    fn file_is_ignored<'a>(
        path: &Path, 
        parsed_args: &ParsedArgs 
    ) -> bool {
        
        let ext = get_extension(&path);
        let ext_str: &str = &ext;
        let mut should_ignore: bool = match &parsed_args.exclude_file_types {
            Some(e) => {
                if ext.len() > 0 && e.contains(&ext_str) {
                    true     
                } else {
                    false
                }
            },
            None => false
        };
        if !should_ignore {  // Because it's not been excluded
            if let Some(i) = &parsed_args.include_file_types {
                if ext.len() > 0 && i.contains(&ext_str) {
                    should_ignore = false 
                } else {
                    should_ignore = true
                }
            }
        };
        should_ignore
    }

    fn get_extension(path: &Path) -> String {
        let ext = path.extension();
        if let Some(p) = ext {
            if let Some(d) = p.to_str() {
                return d.to_string() 
            }
        }
        "".to_string()
    }

    fn highlight_matches(
        parsed_args: &ParsedArgs,
        file_name: String,
        matches: Vec<(usize, String, usize, usize)>
    ) -> usize {

        let mut start_msg: String = format!(
            "\x1b[1;4;35m{}:\x1b[0m", 
            file_name 
        );

        if parsed_args.count_only {
            let spaces: String = " ".repeat(
                min(50, 50 - format!("{}", file_name).len())
            ); 
            start_msg.push_str(&format!(" {}-> {}", spaces, matches.len())); 
            println!("{start_msg}");
            return matches.len()
        };
                
        println!("\x1b[32mMatches in {start_msg}"); 
 
        let max_num_spaces: usize = match matches.last() {
            Some(x) => x.0.to_string().len(),
            None => 0 
        };

        let mut last_line_num: usize = 0;

        for line_of_text in &matches {

            let mut message_text: String = String::new();
            
            let line_num = line_of_text.0;
            let line = &line_of_text.1;
            let match_start = line_of_text.2;
            let match_end = line_of_text.3;
           
            let num_spaces: usize = line_num.to_string().len();
            let padding: String = " ".repeat(max_num_spaces - num_spaces + 1); 

            if last_line_num > 0 && line_num - last_line_num > 1 {
                message_text.push_str("\x1b[1;35m ...\x1b[0m\n");
            };

            message_text.push_str(
                &format!(
                    "\x1b[36m{}{}| \x1b[0m", 
                    padding,
                    line_num,
                )
            );

            message_text.push_str(&line[0..match_start]);
            let matched_word = &line[match_start .. match_end];

            message_text.push_str("\x1b[1;33m"); // yellow
            message_text.push_str(matched_word);
            message_text.push_str("\x1b[0m");
            message_text.push_str(&line[match_end..]);
            println!("{message_text}");
       
            last_line_num = line_num;
        }
        println!(""); 
        matches.len()
    }
    
    fn is_binary(reader: &mut BufReader<fs::File>) -> std::io::Result<bool> {
        // Peek at the internal buffer
        let buf = reader.fill_buf()?;
        Ok(buf.contains(&0))
    }

    fn search<'a>(
        query: &str, 
        reader: &mut BufReader<fs::File>,
        parsed_args: &ParsedArgs
    ) -> Vec<(usize, String, usize, usize)> {

        let mut after_context = 0;
        let mut before_context:
            VecDeque<(usize, String, usize, usize)> = VecDeque::new();

        let mut matching_phrases = Vec::new(); 
        
        for (idx, line) in reader.lines().enumerate() {

            let line = match line {
                Ok(l) => {
                    match parsed_args.trim {
                        true => l.trim().to_string(),
                        false => l 
                    }
                },
                Err(_) => continue
            };

            let (
                has_match, 
                start_idx, 
                end_idx
            ) = match parsed_args.case_sensitive {
                true => {
                    let m = line.contains(query);
                    let i = match line.find(query) {
                        Some(v) => v,
                        None => 0
                    };
                    let e = i + query.len();
                    (m, i, e)
                },
                false => {
                    let lower_line = line.to_lowercase();
                    let lower_q = &query.to_lowercase();
                    let m = lower_line.contains(lower_q);
                    let i = match lower_line.find(lower_q) {
                        Some(v) => v,
                        None => 0
                    };
                    let e = i + query.len();
                    (m, i, e) 
                }
            };

            if has_match {
                for _ in 0..before_context.len() {
                    if let Some(r) = before_context.pop_front() {
                        matching_phrases.push(r)
                    };
                };
                matching_phrases.push((idx + 1, line, start_idx, end_idx));
                after_context = parsed_args.context_after;
            
            } else {
                if after_context > 0 {
                    matching_phrases.push((idx + 1, line, 0, 0));
                    after_context -= 1;
                }
                else if parsed_args.context_before > 0 {
                    before_context.push_back((idx + 1, line, 0, 0));
                    if before_context.len() > parsed_args.context_before {
                        before_context.pop_front();
                    }
                }
            } 
        }
        
        matching_phrases
    }

    fn scan_file_for_matches(
        search_path: &Path,
        parsed_args: &ParsedArgs
    ) -> Result<usize, ErrorResponse> {

        if file_is_ignored(search_path, &parsed_args) {
            return Ok(0)    
        }; 

        let file = match fs::File::open(&search_path) {
            Ok(f) => f,
            Err(msg) => {
                return Err(
                    ErrorResponse {
                        error_msg: format!(
                            "File open failed: {}: {}",
                            &search_path.display(),
                            msg
                        ),
                        error_type: PscanError::FileRead 
                    }
                );
            } 
        };

        let mut reader: BufReader<_> = BufReader::new(file);
      
        if let Ok(b) = is_binary(&mut reader) { 
            if b  && parsed_args.binary_ok { return Ok(0) } 
        };

        let matches = search(
            parsed_args.query, 
            &mut reader, 
            parsed_args
        );
        
        let num_matches = matches.len(); 
        
        if num_matches > 0 {
            let file_name: String = format!("{}", &search_path.display());
            highlight_matches(&parsed_args, file_name, matches);
        };

        Ok(num_matches)

    }

    fn walk(
        scan_path: &Path, 
        parsed_args: &ParsedArgs
    ) -> Result<usize, ErrorResponse> {
        
        let mut total_matches_found: usize = 0;

        if scan_path.is_dir() { 
          
            let entries = match fs::read_dir(&scan_path) {
                Ok(en) => en,
                Err(msg) => {
                    
                    let err_msg: String = format!(
                        "Failed to read {}: {}", 
                        &scan_path.to_string_lossy(),
                        msg
                    ); 
                    
                    return Err(
                        ErrorResponse {
                            error_msg: err_msg,
                            error_type: PscanError::FileRead 
                        }
                    );
                }
            };
            
            for entry_result in entries {
               
                if let Ok(entry_result) = entry_result {

                    let this_path = entry_result.path();

                    if this_path.is_dir() {
                        let result = walk(&this_path, parsed_args);
                        match result {
                            Ok(i) => total_matches_found += i,
                            Err(_) => {
                                return result;
                            }
                        }

                    } else {
                      
                        let result = scan_file_for_matches(
                            &this_path, 
                            parsed_args
                        );
                       
                        match result {
                            Ok(i) => total_matches_found += i,
                            Err(error) => {
                                return Err(error)
                            }
                        }
                    }
                }
            }
        
        } else if scan_path.is_file() {
            
            let ext = get_extension(&parsed_args.path);
            let ext_str: &str = &ext;
            
            if let Some(e) = &parsed_args.exclude_file_types {
                if ext.len() > 0 && e.contains(&ext_str) {
                    return Ok(0) 
                } 
            };

            let result = scan_file_for_matches(
                &scan_path, 
                parsed_args 
            );

            if let Err(_) = result {
                return result 
            }
        };

        Ok(total_matches_found)

    }
    
    //=======================================================================//
    // -------------------------- LOGIC STARTS HERE ------------------------ //
    //=======================================================================// 
    // Check for valid path and query
    if parsed_args.path.as_path().is_dir() {
      
        let mut total_matches_found: usize = 0;

        if !&parsed_args.recursive {
            let err_msg = String::from(
                "Can't search paths without recursion enabled"
            );
            return Err(
                ErrorResponse {
                    error_msg: err_msg,
                    error_type: PscanError::Argument 
                }
            );            
        }

        match fs::read_dir(&parsed_args.path) {
            
            Ok(entries) => {

                for entry_result in entries {
                    
                    if let Ok(entry) = entry_result {
                        
                        let this_path = entry.path();

                        let result = if this_path.is_dir() {
                            walk(&this_path, parsed_args)

                        } else {

                            scan_file_for_matches(&this_path, parsed_args)

                        };
                        
                        match result {
                            Ok(x) => { 
                                total_matches_found += x; 
                            },
                            Err(err) => { 
                                return Err(err) 
                            }
                        }
                    }
                };

                Ok(total_matches_found)

            },
            Err(msg) => {
                Err(
                    ErrorResponse::new(
                        format!(
                            "Could not process {}: {}", 
                            parsed_args.path.as_path().to_string_lossy(), 
                            msg
                        ),
                        PscanError::FileRead 
                    )
                )
            }
        }

    } else if parsed_args.path.as_path().is_file() {
        scan_file_for_matches(&parsed_args.path, &parsed_args)
    
    } else {
        Err(
            ErrorResponse::new(
                String::from("Path is not a file or directory"),
                PscanError::FileRead
            )
        )
    }
}


pub fn run(input_args: Option<Vec<String>>) -> Result<usize, ErrorResponse> {

    // Verify correct input
    let args: Vec<String> = match input_args {
        Some(a) => a,
        None => env::args().skip(1).collect()
    }; 
  
    let parsed_args = match arguments::parse(&args) {
        Ok(c) => c,
        Err(msg) => {
            return Err(
                ErrorResponse {
                    error_msg: format!("{}", msg),
                    error_type: PscanError::InputError 
                }
            )
        }
    };

    if parsed_args.help {
        println!("{HELP_TEXT}");
        return Ok(0)
    } 
    else if parsed_args.show_args {
        println!("{}", parsed_args);
    };
    process_paths_from_args(&parsed_args)

}


// Testing
#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    #[should_panic]
    fn try_a_directory_without_recursion() {
        let search_path: String = String::from("src/text_files");
        let search_query: String = String::from("mary");
        let status = run(Some(vec![search_path, search_query])).is_ok();
        assert!(status);
    }

    #[test]
    fn recursion_test() {
        let params = String::from("-r");
        let search_path = String::from("src/text_files");
        let search_query = String::from("mary");
        let input_args = Some(vec![
            params, search_path, search_query
        ]);
        let result = match run(input_args) {
            Ok(matches) => {
                matches
            },
            Err(_) => 0
        };
        assert!(result > 0); 
    }

    #[test]
    fn case_sensitive_test() {
        let params: String = String::from("-i");
        let search_path: String = String::from("src/text_files/mary.txt");
        let search_query: String = String::from("mary");
        let input_args = Some(vec![params, search_path, search_query]);
        let result = match run(input_args) {
            Ok(x) => x,
            Err(_) => 0
        };
        assert!(result > 0); 
    }

    #[test]
    fn throw_argument_error() {
        let args = Some(vec![
            "src/text_files".to_string(),
            "mary".to_string(),
        ]);
    
        let result = run(args);
        assert!(result.is_err());  // Should fail when no -r on directory
   
        if let Err(err) = result {
            assert!(matches!(err.error_type, PscanError::Argument));
        }
    }
    
    #[test]
    fn throw_file_error() {
        let args = Some(vec![
            "src/text_files/non_existent.txt".to_string(),
            "mary".to_string(),
        ]);
    
        let result = run(args);
        assert!(result.is_err());  // Should fail when no -r on directory
   
        // Optional: check error type
        if let Err(err) = result {
            assert!(matches!(err.error_type, PscanError::FileRead));
        }
    }
}


