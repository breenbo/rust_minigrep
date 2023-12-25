use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error
    let content = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &content) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &content) {
            println!("{line}");
        }
    }

    // no need to return the value if success, just need error management
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
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
        // get ignore_case from env var
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // if no error, return the Config struct
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
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

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}

//
//
//
//
//
// TDD
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        // assertion for testing
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        // assertion for testing
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}
