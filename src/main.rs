use std::{env, error::Error, fs, process};

fn main() {
    // store arguments in vector
    let args: Vec<String> = env::args().collect();

    // handle Result returned by build config: use closure and unwrap_or_else
    // let config = Config::build(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        // store config in config, if error display message error and exit the program
        println!("Problem passing argument: {err}");
        process::exit(1);
    });

    println!("DEBUG for query: {}", config.query);

    // get content in file
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error
    let contents = fs::read_to_string(config.file_path)?;
    println!("With content: \n{}", contents);

    // no need to return the value if success, just need error management
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // error handling: return error if not enough arguments, return Config else
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        // store arguments in variables
        let query = args[1].clone();
        let file_path = args[2].clone();
        //
        println!("Searching for {}", query);
        println!("In file {}", file_path);

        // if no error, return the Config struct
        Ok(Config { query, file_path })
    }
}
