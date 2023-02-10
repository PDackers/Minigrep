use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        /*
            The first element is traditionally the path of the executable, but it can be set to arbitrary text,
            and might not even exist. This means this property should not be relied upon for security purposes.
            This means you need at least three arguments to make this program work, with an optional fourth.
        */
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        let mut ignore_case: bool = false; // Default value.

        /*
            If there are more than four arguments given, ignore all but the fourth.
            Match the fourth, and if it's the right argument, enable case-insensitive search.
            If the fourth argument doesn't match, read the environment variable: IGNORE_CASE.
        */
        if args.len() >= 4 {
            let ic: String = args[3].clone();

            match ic.as_str() {
                "ignore" => {
                    ignore_case = true;
                },
                 _ => {
                    ignore_case = env::var("IGNORE_CASE").is_ok();
                }
            }
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/*
    The following two functions are almost identical. Combine them and provide the
    boolean-value to determine whether the search should be case-sensitive or case-insensitive? 
*/
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// Test Driven Development (TDD).
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query: &str = "duct";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}