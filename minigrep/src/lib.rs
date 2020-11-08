/***
 * 
 * 
    WRITING A FAILING TEST

    The test function specifies the behavior we want the search function to have:
    it will take a query and the text to search for the query in,
    and it will return only the lines from the text that contain the query

    EXPLICIT LIFETIME NOTATION IN THE SEARCH FUNCTION

    Lifetime parameters specify which argument lifetime is connected to the lifetime of the return value.
    In this case, we indicate that the returned vector should contain string slices
    that reference slices of the argument contents (rather than the argument query).

    In other words, the data returned by the search function will live as
    long as the data passed into the search function in the contents argument

    Because contents is the argument that contains all of our text
    and we want to return the parts of that text that match,
    we know contents is the argument that should be connected to the return value using the lifetime syntax

    ITERATING THROUGH THE CONTENTS OF THE FILE

    The lines method returns an iterator

    CASE INSENSITIVE FUNCTION

    Note that query is now a String rather than a string slice,
    because calling to_lowercase creates new data rather than referencing existing data

    When we pass query as an argument to the contains method now,
    we need to add an ampersand because the signature of contains is defined to take a string slice

    ENVIRONMENT VARIABLES

    The functions for working with environment variables are in the env module in the standard library

    Use the var function from the env module to check for an environment variable named CASE_INSENSITIVE

    The env::var function returns a Result that will be the successful Ok variant
    that contains the value of the environment variable if the environment variable is set.
    It will return the Err variant if the environment variable is not set.

    If the CASE_INSENSITIVE environment variable is set to anything,
    is_err will return false and the program will perform a case-insensitive search
***/


use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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
Duct tape";

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

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}