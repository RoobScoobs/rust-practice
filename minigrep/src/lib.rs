/***
 * 
 * 
    WRITING A FAILING TEST

    The test function specifies the behavior we want the search function to have:
    it will take a query and the text to search for the query in,
    and it will return only the lines from the text that contain the query

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
    
    Ok(())
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
        Pick three."

        asser_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}