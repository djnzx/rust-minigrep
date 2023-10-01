use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    println!("=== results ===");

    let r = if config.ignore_case {
        search_case_insensitive(config.query.as_str(), contents.as_str())
    } else {
        search(config.query.as_str(), contents.as_str())
    };
    if r.is_empty() {
        println!("not found")
    } else {
        r.iter().for_each(|x| println!("{}", x));
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut outcome: Vec<&str> = Vec::new();
    for line in contents.lines() {
        println!("searching: {}", line);
        if line.contains(query) {
            println!("found");
            outcome.push(line);
        }
    }
    outcome
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let binding = query.to_lowercase();
    let query = binding.as_str();
    // why doesn't work
    // let query = query.to_lowercase().as_str();

    let mut outcome: Vec<&str> = Vec::new();
    for line in contents.lines() {
        println!("searching: {}", line);
        if line.to_lowercase().contains(query) {
            println!("found");
            outcome.push(line);
        }
    }
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;

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
