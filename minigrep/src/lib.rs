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
    we know contents is the argument that should be connected to the return value using the lifetime syntax.

    ITERATING THROUGH THE CONTENTS OF THE FILE

    The lines method returns an iterator
***/


use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query,
            filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}