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

    ITERATORS

    The standard library documentation for the env::args function shows that the type of the iterator it returns is std::env::Args
    So it's necessary to update the signature of the Config::new function
    The parameter args has the type std::env::Args instead of &[String]

    Because we’re taking ownership of args and we’ll be mutating args by iterating over it,
    we can add the mut keyword

    The standard library documentation also mentions that std::env::Args implements the Iterator trait,
    so we know we can call the next method on it

    Can refactor search function to use iterator adaptor methods (e.g. filter, map)

    Doing so also lets us avoid having a mutable intermediate results vector
    The functional programming style prefers to minimize the amount of mutable state to make code clearer
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
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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