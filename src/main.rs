use pscan::{error_handler, run};
use std::process;


fn main() {
    match run(None) {
        Ok(exit_code) => exit_code,
        Err(response) => {
            process::exit(error_handler(response))
        }
    };
}

