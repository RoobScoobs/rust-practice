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

    std::fs

    we need std::fs to handle files, which takes the filename, opens that file, 
    and returns a Result<String> of the file's contents

    SEPARATION OF CONCERNS IN BINARY PROJECTS

    When the main function starts growing in scope it's best to 
    split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs

    The responsibilities that remain in the main function after this process should be limited to the following:
        - Calling the command line parsing logic with the argument values
        - Setting up any other configuration
        - Calling a run function in lib.rs
        - Handling the error if run returns an error

    In sum, main.rs handles running the program,
    and lib.rs handles all the logic of the task at hand

    TRADE-OFFS OF USING CLONE

    Calling clone() will make a full copy of the data called upon,
    however it takes more time and memory than storing a reference to the data

    There's a tendency to avoid using clone to fix ownership problems because of its runtime cost

    ERROR HANDLING

    A call to panic! is more appropriate for a programming problem than a usage problem.

    So rather than calling the panic macro, we can return a Result that indicates either success or an error

    Successful variant contains an instance of the Config struct
    and the Error variant holds a static lifetime string literal

    USING unwrap_or_else

    It allows us to define some custom, non-panic! error handling.

    If the Result is an Ok value, this method’s behavior is similar to unwrap:
    it returns the inner value Ok is wrapping

    However, if the value is an Err value, this method calls the code in the closure,
    which is an anonymous function we define and pass as an argument to unwrap_or_else

    EXTRACTING FURTHER LOGIC FROM MAIN BY USING A RUN FUNCTION

    Changed the return type of the run function to Result<(), Box<dyn Error>>

    Box<dyn Error> means the function will return a type that implements the Error trait
    but we don’t have to specify what particular type the return value will be

    This gives us flexibility to return error values that may be of different types in different error cases
    
    The dyn keyword is short for “dynamic.”

    Opted to use the ? operator so rather than panic! on an error,
    ? will return the error value from the current function for the caller to handle

     We’ve declared the run function’s success type as () in the signature,
     which means we need to wrap the unit type value in the Ok value

     Ok(()) is an idiomatic way to indicate that we’re calling run for its side effects only;
     it doesn’t return a value we need.

     HANDLING ERRORS RETURNED FROM RUN FUNCTION

     We use if let rather than unwrap_or_else to check whether run returns an Err value
     and call process::exit(1)

     The run function doesn’t return a value that we want to unwrap in the same way that Config::new returns the Config instance

     Because run returns () in the success case, we only care about detecting an error,
     so we don’t need unwrap_or_else to return the unwrapped value because it would only be ()
     
     The `if let` construct reads: "if `let` destructures `run(config)` into
     `Err(e)`, evaluate the block (`{}`).

     WRITING ERROR MESSAGE TO STANDARD ERROR
     
     Command line programs are expected to send error messages to the standard error stream
     so we can still see error messages on the screen even
     if we redirect the standard output stream to a file
     
     The way to demonstrate the standard output behavior is by running the program with > and the filename, output.txt,
     so that it redirects the standard output stream to that particular file
     
     The > syntax tells the shell to write the contents of standard output to output.txt instead of the screen

     The standard library provides the eprintln! macro that prints to the standard error stream
***/

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}