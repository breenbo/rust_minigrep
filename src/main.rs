use minigrep::Config;
use std::{env, process};

fn main() {
    // handle Result returned by build config: use closure and unwrap_or_else
    // let config = Config::build(&args);
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // store config in config, if error display message error and exit the program
        eprintln!("Problem passing argument: {err}");
        process::exit(1);
    });

    // get content in file
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
