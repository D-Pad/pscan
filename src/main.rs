use pscan;
use std::process;


fn main() {
    match pscan::run(None) {
        Ok(exit_code) => exit_code,
        Err(response) => {
            process::exit(pscan::error_handler(response))
        }
    };
}

