use std::path::PathBuf;

pub struct ParsedArgs<'a> {
    pub path: Option<PathBuf>,
    pub query: Option<&'a str>,
    
    pub recursive: bool,
    
    pub case_sensitive: bool,
    
    pub context_before: u8,  
    pub context_after: u8,
    
    pub include_file_types: Option<Vec<&'a str>>,
    pub exclude_file_types: Option<Vec<&'a str>>,

}

impl<'a> ParsedArgs<'a> {
    
    fn new(args: &'a [String]) -> Result<Self, &'static str> {
       
        if args.len() >= 2 {
            
            let mut path: Option<PathBuf> = None;
            let mut query: Option<&'a str> = None;

            let mut recursive: bool = false;
            let mut case_sensitive: bool = true;

            let mut context_before: u8 = 0;
            let mut context_after: u8 = 0;

            let mut include_file_types: Option<Vec<&'a str>> = None;
            let mut exclude_file_types: Option<Vec<&'a str>> = None;

            let mut key: char = '!';

            let mut iter_count = 0;
            while iter_count < args.len() {
              
                let argument = &args[iter_count];
                
                if argument.starts_with('-') {
                 
                    let arg_str = argument.as_str();
                    match arg_str {

                        // Context options
                        "-A"      | "-B"       | "-C" | 
                        "--after" | "--before" | "--context" => { 
                            if let Ok(d) = args[iter_count + 1].parse::<u8>() {
                                match arg_str { 
                                    "-A" | "--after" => { 
                                        context_after = d;
                                    },
                                    "-B" | "--before" => { 
                                        context_before = d;
                                    },
                                    _ => {
                                        context_after = d;
                                        context_before = d;
                                    }
                                }
                            }
                        },

                        "-E" | "--exclude-file-types" => {
                            if exclude_file_types.is_none() {
                                exclude_file_types = Some(Vec::new())
                            };
                            key = 'E';
                        },
                        "-I" | "--include-file-types" => {
                            if include_file_types.is_none() {
                                include_file_types = Some(Vec::new())
                            }
                            key = 'I';
                        },
                        
                        // Single param short options: Ex: -ri
                        _ => {
                            for param in argument.chars() {
                                if param == 'i' { case_sensitive = false }
                                else if param == 'r' { recursive = true }
                            }
                        }
                    }
                } else {
                    
                    if path.is_none() { 
                        path = Some(PathBuf::from(argument.as_str())); 
                    } else if query.is_none() { 
                        query = Some(argument.as_str()) 
                    } else if key != '!' {
                        if key == 'I' {
                            if let Some(ref mut vals) = include_file_types {
                                vals.push(argument);
                            }
                        }
                        else if key == 'E' {
                            if let Some(ref mut vals) = exclude_file_types {
                                vals.push(argument);
                            }
                        }
                    }
                }

                iter_count += 1;
            }

            let parsed_args: ParsedArgs = ParsedArgs {
                query,
                path,
                recursive,
                case_sensitive, 
                context_before, 
                context_after, 
                include_file_types, 
                exclude_file_types
            }; 
            
            Ok(parsed_args)
    
        } else {
            Err("Must pass a search path and phrase, in that order")
        }
    }
}


pub fn parse<'a>(args: &'a [String]) -> Result<ParsedArgs<'a>, &'static str> {
    ParsedArgs::new(&args)
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn no_flags_passed() {
        let args: Vec<String> = vec![
            "./search_path".to_string(),
            "search_query".to_string()
        ];
        let parsed = match ParsedArgs::new(&args) {
            Ok(p) => p,
            Err(_) => panic!("Arg parsing failed")
        };
        assert!(!parsed.recursive); 
        assert!(parsed.case_sensitive); 
        assert_eq!(parsed.context_before, 0); 
        assert_eq!(parsed.context_after, 0);
        assert!(parsed.include_file_types.is_none());
        assert!(parsed.exclude_file_types.is_none());
    }

    #[test]
    fn recursion_flag_passed() {
        let args: Vec<String> = vec![
            "-r".to_string(),
            "./search_path".to_string(),
            "search_query".to_string()
        ];
        let parsed = match ParsedArgs::new(&args) {
            Ok(p) => p,
            Err(_) => panic!("Arg parsing failed")
        };
        assert!(parsed.recursive); 
        assert!(parsed.case_sensitive); 
        assert_eq!(parsed.context_before, 0); 
        assert_eq!(parsed.context_after, 0);
        assert!(parsed.include_file_types.is_none());
        assert!(parsed.exclude_file_types.is_none());
    }


    #[test]
    fn context_after_flag_passed() {
        let args: Vec<String> = vec![
            "./search_path".to_string(),
            "search_query".to_string(),
            "-A".to_string(),
            "3".to_string()
        ];
        let parsed = match ParsedArgs::new(&args) {
            Ok(p) => p,
            Err(_) => panic!("Arg parsing failed")
        };
        assert!(!parsed.recursive); 
        assert!(parsed.case_sensitive); 
        assert_eq!(parsed.context_before, 0); 
        assert_eq!(parsed.context_after, 3);
        assert!(parsed.include_file_types.is_none());
        assert!(parsed.exclude_file_types.is_none());
    }

    #[test]
    fn context_before_flag_passed() {
        let args: Vec<String> = vec![
            "./search_path".to_string(),
            "search_query".to_string(),
            "-B".to_string(),
            "3".to_string()
        ];
        let parsed = match ParsedArgs::new(&args) {
            Ok(p) => p,
            Err(_) => panic!("Arg parsing failed")
        };
        assert!(!parsed.recursive); 
        assert!(parsed.case_sensitive); 
        assert_eq!(parsed.context_before, 3); 
        assert_eq!(parsed.context_after, 0);
        assert!(parsed.include_file_types.is_none());
        assert!(parsed.exclude_file_types.is_none());
    }


    #[test]
    fn context_before_and_after_flag_passed() {
        let args: Vec<String> = vec![
            "./search_path".to_string(),
            "search_query".to_string(),
            "-C".to_string(),
            "3".to_string()
        ];
        let parsed = match ParsedArgs::new(&args) {
            Ok(p) => p,
            Err(_) => panic!("Arg parsing failed")
        };
        assert!(!parsed.recursive); 
        assert!(parsed.case_sensitive); 
        assert_eq!(parsed.context_before, 3); 
        assert_eq!(parsed.context_after, 3);
        assert!(parsed.include_file_types.is_none());
        assert!(parsed.exclude_file_types.is_none());
    }

    #[test]
    fn exclude_file_types_flag_passed() {
        let args: Vec<String> = vec![
            "./search_path".to_string(),
            "search_query".to_string(),
            "-E".to_string(),
            ".py".to_string()
        ];
        let parsed = match ParsedArgs::new(&args) {
            Ok(p) => p,
            Err(_) => panic!("Arg parsing failed")
        };
        assert!(!parsed.recursive); 
        assert!(parsed.case_sensitive); 
        assert_eq!(parsed.context_before, 0); 
        assert_eq!(parsed.context_after, 0);
        assert!(parsed.include_file_types.is_none());
        assert!(parsed.exclude_file_types.is_some());
        assert!(parsed.exclude_file_types.unwrap().len() > 0);
    }

    #[test]
    fn include_file_types_flag_passed() {
        let args: Vec<String> = vec![
            "./search_path".to_string(),
            "search_query".to_string(),
            "-I".to_string(),
            ".sh".to_string()
        ];
        let parsed = match ParsedArgs::new(&args) {
            Ok(p) => p,
            Err(_) => panic!("Arg parsing failed")
        };
        assert!(!parsed.recursive); 
        assert!(parsed.case_sensitive); 
        assert_eq!(parsed.context_before, 0); 
        assert_eq!(parsed.context_after, 0);
        assert!(parsed.include_file_types.is_some());
        assert!(parsed.include_file_types.unwrap().len() > 0);
        assert!(parsed.exclude_file_types.is_none());
    }
}


