use std::{fs, env, path, io};

use crate::arguments::parser::ParsedArgs;
pub mod arguments;


pub enum PscanError {
    FileRead,
    Argument,
    InputError
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
            err_msg.push_str("BadArgumentError: ");
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

    fn highlight_matches(query: &str, matches: Vec<(usize, &str)>) -> usize {

        let max_num_spaces: usize = match matches.last() {
            Some(x) => x.0.to_string().len(),
            None => 0 
        };

        // let padding: String = match matches.last() {
        //     Some(x) => " ".repeat(x.0.to_string().len()),
        //     None => " ".to_string()
        // };

        for line_of_text in &matches {

            let mut message_text: String = String::new();
            let mut last = 0;
            
            let line = line_of_text.1;
            let line_num = line_of_text.0;
            
            let lower_line = line.to_lowercase();
            let lower_query = query.to_lowercase(); 
          
            let num_spaces: usize = line_num.to_string().len();
            let padding: String = " ".repeat(max_num_spaces - num_spaces + 1); 

            message_text.push_str(&format!(
                    "\x1b[36m{}{}| \x1b[0m", 
                    padding,
                    line_num,
                )
            );

            for (idx, _) in lower_line.match_indices(&lower_query) {

                message_text.push_str(&line[last..idx]);
              
                let matched_word = &line[idx .. idx + query.len()];

                message_text.push_str("\x1b[1;33m"); // yellow
                message_text.push_str(matched_word);
                message_text.push_str("\x1b[0m");
                last = idx + query.len();

            }

            message_text.push_str(&line_of_text.1[last..]);
            println!("{message_text}");
        }
        println!(""); 
        matches.len()
    }


    fn search<'a>(
        query: &str, 
        contents: &'a str,
        case_sensitive: bool
    ) -> Vec<(usize, &'a str)> {

        let mut matching_phrases = Vec::new(); 
        
        for (idx, line) in contents.lines().enumerate() {

            let mut line_contains_match: bool = false;

            if case_sensitive {
                if line.contains(query) {
                    line_contains_match = true;
                }
            } else {
                if line.to_lowercase().contains(&query.to_lowercase()) {
                    line_contains_match = true;
                } 
            }

            if line_contains_match {
                matching_phrases.push((idx + 1, line));
            } 

        }
        
        matching_phrases
    }

    fn scan_file_for_matches(
        search_path: &str, 
        search_query: &str, 
        case_sensitive: bool
    ) -> Result<usize, ErrorResponse> {

        let file_contents = match fs::read_to_string(&search_path) {
            Ok(text) => text,
            Err(msg) => {
                
                if msg.kind() != io::ErrorKind::InvalidData {
                    return Err(
                        ErrorResponse {
                            error_msg: format!(
                                "File read failed: {}: {}",
                                &search_path,
                                msg
                            ),
                            error_type: PscanError::FileRead 
                        }
                    );
                } else {

                    // Skip binary files, and files that are non UTF-8
                    return Ok(0);
                }
            } 
        };

        let matches = search(search_query, &file_contents, case_sensitive);
        let num_matches = matches.len(); 
        if num_matches > 0 { 
            println!("\x1b[32mMatches found in {}:\x1b[0m", &search_path); 
            highlight_matches(search_query, matches);
        }

        Ok(num_matches)

    }

    fn walk(
        scan_path: &path::Path, 
        query: &str, 
        case_sensitive: bool) -> Result<usize, ErrorResponse> {
      
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
                        let result = walk(&this_path, query, case_sensitive);
                        match result {
                            Ok(i) => total_matches_found += i,
                            Err(_) => {
                                return result;
                            }
                        }

                    } else {
                        
                        let result = scan_file_for_matches(
                            &this_path.to_string_lossy(), 
                            query,
                            case_sensitive
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
            let result = scan_file_for_matches(
                &scan_path.to_string_lossy().as_ref(), 
                query,
                case_sensitive
            );

            if let Err(_) = result {
                return result 
            }
        };

        Ok(total_matches_found)

    }

    // Check for valid path and query
    let search_path = match parsed_args.path {
        Some(p) => p, 
        None => {

            let err_msg = String::from(
                "ArgumentError: Must pass a root path to search"
            );
            
            return Err(
                ErrorResponse {
                    error_msg: err_msg,
                    error_type: PscanError::Argument 
                }
            )
        }
    };

    let search_query = match parsed_args.query {
        Some(q) => q, 
        None => {
            let err_msg = String::from("Must pass search query");
            return Err(
                ErrorResponse {
                    error_msg: err_msg,
                    error_type: PscanError::Argument 
                }
            );       
        }
    };

    // Read the file
    let root_path: &path::Path = path::Path::new(&search_path);
    if root_path.is_dir() {
      
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

        match fs::read_dir(&root_path) {
            
            Ok(entries) => {

                for entry_result in entries {
                    
                    if let Ok(entry) = entry_result {
                        
                        let this_path = entry.path();
   
                        let result = if this_path.is_dir() {
                            walk(
                                &this_path, 
                                search_query, 
                                parsed_args.case_sensitive
                            )

                        } else {
                            scan_file_for_matches(
                                &this_path.to_string_lossy(), 
                                search_query,
                                parsed_args.case_sensitive
                            )

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
                        format!("Could not process {}: {}", &search_path, msg),
                        PscanError::FileRead 
                    )
                )
            }
        }

    } else if root_path.is_file() {
        let case: bool = parsed_args.case_sensitive;
        scan_file_for_matches(search_path, search_query, case)
    
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
        None => env::args().collect()
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

    process_paths_from_args(&parsed_args)

}


// Testing
#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    #[should_panic]
    fn try_a_directory_without_recursion() {
        let search_path: String = String::from("./text_files");
        let search_query: String = String::from("mary");
        let status = run(Some(vec![search_path, search_query])).is_ok();
        assert!(status);
    }

    #[test]
    fn recursion_test() {
        let params: String = String::from("-r");
        let search_path: String = String::from("./text_files");
        let search_query: String = String::from("Mary");
        let input_args = Some(vec![params, search_path, search_query]);
        let result = match run(input_args) {
            Ok(matches) => matches,
            Err(_) => 0
        };    
        assert!(result > 0); 
    }

    #[test]
    fn case_sensitive_test() {
        let params: String = String::from("-i");
        let search_path: String = String::from("./text_files/mary.txt");
        let search_query: String = String::from("mary");
        let input_args = Some(vec![params, search_path, search_query]);
        let result = match run(input_args) {
            Ok(x) => x,
            Err(_) => 0
        };
        assert!(result > 0); 
    }

    #[test]
    fn directory_without_recursion_should_fail() {
        let args = Some(vec![
            "./text_files".to_string(),
            "mary".to_string(),
        ]);
    
        let result = run(args);
        assert!(result.is_err());  // Should fail when no -r on directory
    
        // Optional: check error type
        if let Err(err) = result {
            assert!(matches!(err.error_type, PscanError::Argument));
            assert!(err.error_msg.contains("recursion"));
        }
    }

}


