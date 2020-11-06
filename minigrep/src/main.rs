/***
 * 
 * 
    Minigrep Program From The Rust Book

    Able to run the CLI with cargo run, a string to search for, and a path to a file to search in, like so:
    
    cargo run searchstring example-filename.txt

    To read the values of command line arguments we pass to it,
    we’ll need a function provided in Rust’s standard library,
    which is std::env::args

    COLLECT function

    We can use the collect function to create many kinds of collections,
    so we explicitly annotate the type of args to specify that we want a vector of strings.
    Although we very rarely need to annotate types in Rust,
    collect is one function you do often need to annotate
    because Rust isn’t able to infer the kind of collection you want.

***/

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
