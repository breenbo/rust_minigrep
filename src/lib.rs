use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error
    let content = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &content) {
        println!("{line}");
    }

    // no need to return the value if success, just need error management
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // error handling: return error if not enough arguments, return Config else
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        // store arguments in variables
        let query = args[1].clone();
        let file_path = args[2].clone();

        // if no error, return the Config struct
        Ok(Config { query, file_path })
    }
}

// 'a: lifetime of the return value, which is bound to the content
// (return value is a part of content, so need to keep content alive until the end)
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // 1. iterate through each line of content
    for line in content.lines() {
        // 2. check if query present in line
        if line.contains(query) {
            // 3. if yes => add it to the list of returned values
            results.push(line);
        }
        // 4. if no => do nothing
    }

    // 5. return list of values
    results
}

//
// TDD
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        // assertion for testing
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
