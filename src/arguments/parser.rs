use std::{path::PathBuf, fmt};


pub const HELP_TEXT: &str = r#"
pscan - A grep-like tool with detailed match location and context lines

USAGE:
    pscan [OPTIONS] <PATH> <QUERY> [-I ext1 ext2 ...] [-E ext1 ext2 ...]

POSITIONAL ARGUMENTS:
    PATH                Directory or file to search (required)
    QUERY               Text or pattern to search for (required)

SEARCH OPTIONS:
    -i                  Perform case-insensitive matching (default is case-sensitive)
    -r                  Search directories recursively
    -s                  Show parsed arguments before starting search (useful for debugging)
    -t                  Trims white space from any matching lines
    -b                  Enables binary file reading.
    -c                  Only shows the number of matches found per file

CONTEXT CONTROL:
    -A, --after N       Print N lines of trailing context after each match
    -B, --before N      Print N lines of leading context before each match
    -C, --context N     Print N lines of context both before and after each match
                        (equivalent to -B N -A N)

FILE TYPE FILTERING:
    -I, --include-file-types ext1 ext2 ...
                        Only search files whose extension is in the list
                        (e.g. -I rs toml yaml)
    -E, --exclude-file-types ext1 ext2 ...
                        Skip files whose extension is in the list. Will also skip 
                        directories if the directory name is included here.
                        (e.g. -E jpg png gif node_modules)

EXAMPLES:
    pscan ./src "println!"
    pscan -i -r . "error handling"
    pscan -C 2 Cargo.toml "version"
    pscan -I rs toml -r src "unsafe"
    pscan --before 1 --after 3 logs "ERROR"

NOTE:
    Short options can be combined: -ris is equivalent to -r -i -s
    Extensions for -I/-E should be given without leading dot
"#; 


pub struct ParsedArgs<'a> {
    
    pub path: PathBuf,
    pub query: &'a str,
    
    pub recursive: bool,
    pub case_sensitive: bool,
    pub show_args: bool,
    pub trim: bool,
    pub binary_ok: bool,
    pub count_only: bool,
    
    pub context_before: usize,  
    pub context_after: usize,

    pub include_file_types: Option<Vec<&'a str>>,
    pub exclude_file_types: Option<Vec<&'a str>>,

    pub help: bool

}

impl<'a> fmt::Display for ParsedArgs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[1;36mParsed Arguments: \x1b[0m\x1b[1m{{\x1b[0m")?;
        write!(f, "\n  \x1b[33mpath:              \x1b[0m {}",
            self.path.display())?;
        write!(f, "\n  \x1b[33mquery:             \x1b[0m {}",
            self.query)?;
        write!(f, "\n  \x1b[33mrecursive:     \x1b[0m     {}",
            self.recursive)?;
        write!(f, "\n  \x1b[33mtrim:          \x1b[0m     {}",
            self.trim)?;
        write!(f, "\n  \x1b[33mbinary_ok:     \x1b[0m     {}",
            self.binary_ok)?;
        write!(f, "\n  \x1b[33mcase_sensitive:\x1b[0m     {}", 
            self.case_sensitive)?;
        write!(f, "\n  \x1b[33mcount_only:    \x1b[0m     {}", 
            self.count_only)?;
        write!(f, "\n  \x1b[33mcontext_before:\x1b[0m     {}",
            self.context_before)?;
        write!(f, "\n  \x1b[33mcontext_after:\x1b[0m      {}",
            self.context_after)?;
        if let Some(t) = &self.include_file_types {
            write!(f, "\n  \x1b[33minclude_file_types:\x1b[0m {:?}", t)?;
        };
        if let Some(t) = &self.exclude_file_types {
            write!(f, "\n  \x1b[33mexclude_file_types:\x1b[0m {:?}", t)?;
        };
        write!(f, "\n  \x1b[33mshow_args:\x1b[0m          {}",
            &self.show_args)?;
        write!(f, "\n\x1b[1m}}\x1b[0m")
    }
}

impl<'a> ParsedArgs<'a> {
    
    fn new(args: &'a [String]) -> Result<Self, &'static str> {

        if args.len() == 1 && args.contains(&"--help".to_string()) {
            Ok(ParsedArgs {
                path: PathBuf::from(""),
                query: "",
                recursive: false,
                case_sensitive: false,
                show_args: false,
                trim: false, 
                binary_ok: false,
                count_only: false,
                context_before: 0, 
                context_after: 0,
                include_file_types: None,
                exclude_file_types: None,
                help: true 
            })
        }
        else if args.len() >= 2 {
            
            let mut path: PathBuf = PathBuf::from("");
            let mut query: &'a str = "";

            let mut recursive: bool = false;
            let mut show_args: bool = false;
            let mut trim: bool = false;
            let mut binary_ok: bool = false;
            let mut count_only: bool = false;
            let mut case_sensitive: bool = true;

            let mut context_before: usize = 0;
            let mut context_after: usize = 0;
            
            let mut include_file_types: Option<Vec<&'a str>> = None;
            let mut exclude_file_types: Option<Vec<&'a str>> = None;

            let mut key: char = '!';

            let mut help: bool = false;

            let mut iter_count = 0;
            while iter_count < args.len() {
              
                let argument = &args[iter_count];
                
                if argument.starts_with('-') {
                 
                    let arg_str = argument.as_str();
                    key = '!';
                    match arg_str {

                        // Context options
                        "-A"      | "-B"       | "-C" | 
                        "--after" | "--before" | "--context" => { 
                            let n = args[iter_count + 1].parse::<usize>();
                            if let Ok(d) = n {
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
                        
                        // Print the help menu and exit
                        "--help" => { help = true; break } 

                        // Single param short options: Ex: -ri
                        _ => {
                            for param in argument.chars() {
                                if param == 'i' { case_sensitive = false }
                                else if param == 'r' { recursive = true }
                                else if param == 's' { show_args = true }
                                else if param == 't' { trim = true }
                                else if param == 'b' { binary_ok = true }
                                else if param == 'c' { count_only = true }
                            }
                        }
                    }
                } else {
                    
                    if key != '!' {
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
                    else if path.as_os_str().is_empty() { 
                        path = PathBuf::from(argument.as_str()); 
                    }
                    else if query == "" { 
                        query = argument.as_str() 
                    } 
                }

                iter_count += 1;
            }

            if path.as_os_str().is_empty() {
                Err("ArgumentError: Must pass a root path to search")
            }
            else if query == "" {
                Err("ArgumentError: Must pass a search query")
            }
            else {
                Ok(ParsedArgs {
                    query,
                    path,
                    recursive,
                    case_sensitive, 
                    show_args,
                    trim,
                    binary_ok,
                    count_only,
                    context_before, 
                    context_after, 
                    include_file_types, 
                    exclude_file_types,
                    help, 
                }) 
            }
        
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


