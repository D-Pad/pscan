

pub struct ParsedArgs<'a> {
    pub query: Option<&'a str>,
    pub path: Option<&'a str>,
    pub recursive: bool,
    pub case_sensitive: bool
}

impl<'a> ParsedArgs<'a> {
    
    fn new(args: &'a [String]) -> Result<Self, &'static str> {
        
        if args.len() > 2 {
            
            let mut path: Option<&'a str> = None;
            let mut query: Option<&'a str> = None;

            let mut recursive: bool = false;
            let mut case_sensitive: bool = true;
          
            for argument in args.iter().skip(1) {
               
                if argument.starts_with('-') {
                    
                    for param in argument.chars() {
                        if param == 'i' { case_sensitive = false }
                        else if param == 'r' { recursive = true }
                    }
                
                } else {
                    
                    if path.is_none() { 
                        path = Some(argument.as_str()) 
                    
                    } else if query.is_none() { 
                        query = Some(argument.as_str()) 
                    }

                }
            }

            let parsed_args: ParsedArgs = ParsedArgs {
                query,
                path,
                recursive,
                case_sensitive
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


