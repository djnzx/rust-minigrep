use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn make(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Two parameters are expected: substring and file name.");
        }

        // TODO: access it in main
        let ignore_case = env::var("IGNORE_CASE").is_ok(); // set to whatever (just present)

        let query = args[1].clone();
        let file_path = args[2].clone();
        let c = Config {
            query,
            file_path,
            ignore_case,
        };
        Ok(c)
    }
}

///                                  covariance ???
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    /// `?` in the end allows to shortcut Result to exit,
    /// but it should match the signature
    /// to make it covariant value is wrapped into the Box
    /// TODO: learn how to handle in the incremental way, line by line
    let contents = fs::read_to_string(config.file_path)?;
    // getOrThrow semantics
    // .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let found = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    let mut is_found = false;
    for line in found {
        is_found = true;
        println!("Found: `{line}`");
    }

    if !is_found {
        println!("Nothing found.");
    }

    Ok(())
}

/// In other words, we tell Rust that the data `returned` by the `search` function
/// will live as long as the data passed into the search function in the `contents` argument
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /// the best stub for non-implemented functions
    // panic!("to be implemented")
    let mut outcome = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            outcome.push(line);
        }
    }

    outcome
}

pub fn search_functional<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /// the best stub for non-implemented functions
    // panic!("to be implemented")
    let query = query.to_lowercase();

    let mut outcome = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            outcome.push(line);
        }
    }

    outcome
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // TODO: how to write well formatted multiline literals
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let contents = dbg!(contents);

        let excerpt = vec!["safe, fast, productive."];
        assert_eq!(excerpt, search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
