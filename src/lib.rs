pub mod config;

use crate::config::Config;
use colored::*;
use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let read_file = fs::read_to_string(config.file_path)?;

    let result = if config.case_insensitive {
        search_case_insensitive(&config.query, &read_file)
    } else {
        search(&config.query, &read_file)
    };

    for line in result {
        let highlighted_line = if config.case_insensitive {
            line.replace(
                &config.query.to_lowercase(),
                &config.query.red().bold().to_string(),
            )
        } else {
            line.replace(&config.query, &config.query.red().bold().to_string())
        };
        println!("{}", highlighted_line);
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }

    result
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
