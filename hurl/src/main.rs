/***
 * 
 * 
    COMMAND LINE APPLICATION

    Building a simple command line application for making HTTP requests
    with the following basic feature set:
        - parse the response if it is JSON
        - print out the information about the request including headers
        - print out the response as nicely as possible

    Make a get request to example.com:
        
        hurl example.com

    Make a POST request to example.com with a data JSON object as 
        {
            "foo": "bar"
        }
    
        hurl example.com foo=bar

    Make a PUT request to example.com using HTTPS,
    set the X-API-TOKEN header to abc123,
    and upload a file named foo.txt with the form field name info:

        hurl -s -f PUT example.com X-API-TOKEN:abc123 info@foo.txt

    The -s for --secure option used to turn on HTTPS
    and -f option used to turn on sending as a form

    Headers are to be specified with a colon separating the name from the value

    Files are to be uploaded by using @ between the form field name
    and the path to the file on disk

    Make a form POST request to example.com,
    use bob as the username for basic authentication,
    set the query parameter of foo equal to bar,
    and set the form field option equal to all:

        hurl -f -a bob POST example.com foo==bar option=all

        *before making the request,
        the user should be prompted for their password
        which should be used for the basic authentication scheme*

    QUICK OVERVIEW OF DEPENDENCIES

    structopt - command line argument parsing and much more
    heck - converting strings to difference cases (e.g. snake_case, camelCase)
    log - logging facade for outputting verbose information
    pretty_env_logger - logging implementation that works with log
    serde - (de)serialization of data
    serde_json - serde for JSON data
    reqwest - HTTP client
    rpassword - ask a user for a password without echoing it to the terminal

    IMPORTS

    The structopt crate defines a trait StructOpt and a custom derive
    which allows you to derive that trait for a type you create

    These two pieces together create a system for declaratively specifying how your application takes input from the command line

    TitleCase is also a trait from the heck crate
    so that we can convert strings into their title cased equivalents
    For example, this_string would be converted to ThisString

    The import of the trace macro from the log crate is
    to leverage the log crate’s nice features to implement a form of verbose logging

    When writing a library or other tool that wants to generate logs,
    use the macros in the log crate to write those messages at various verbosity levels

    MODULES

    Splitting up the functionality into different modules:
        - app
        - client
        - errors

    TYPE ALIAS

    One feature to support is pretty printing JSON responses with ordered keys

    Rust has multiple hash map implementations in the standard library,
    one in particular is the BTreeMap which stores entries sorted by the key

    MAIN FN

    Return a Result, in particular a custom HurlResult with a success type of ()
    meaning the only meaningful value to report is for errors,
    and that the Ok case just means everything worked

    This shows that the HurlResult type that needs to be written is generic over the success type

    The first line of the main function uses a call to from_args
    which is defined on the StructOpt trait
    Therefore, a struct called App needs to be created in the app module
    which implements this trait
    and it will handle all of the command line argument parsing

    Then there's a call to validate which uses the ? operator to exit early if this function fails

    This is not part of the argument parsing that StructOpt does
    Rather, this is for handling certain constraints on the arguments that StructOpt is unable to enforce

    LOGGING

    Using the log_level method on the app will get a value to set up logging

    The pretty_env_logger crate uses environment variables to configure what to print,
    so the RUST_LOG environment variable is explicitly set to the value acquired

    The format is RUST_LOG=binary_name=level
    where binary_name is the name of the binary
    and level is one of the five level values that log defines: trace, debug, info, warn, and error

    CORE OF THE APP

    Use the cmd (short for command), property on our app to direct what type of request to make

    There are two cases, either there's a command which specifies the HTTP verb to use,
    in that case the client module will make the request
    and then call a handle_response function with the result

    If there's no command, i.e. app.cmd matches None,
    then the default case is triggered which just got a URL

    In this case, a GET request is executed if there aren't any  data arguments,
    otherwise make a POST request

    Then also call a method on the client module to make this request
    and pipe through to the same handle_response function
***/

use structopt::StructOpt;
use heck::TitleCase;
use log::trace;

mod app;
mod client;
mod errors;

use errors::HurlResult;

type OrderedJson = std::collections::BTreeMap<String, serde_json::Value>;

fn main() -> HurlResult<()> {
    let mut app = app::App::from_args();
    app.validate()?;

    if let Some(level) = app.log_level() {
        std::env::set_var("RUST_LOG", format!("hurl={}", level));
        pretty_env_logger::init();
    }

    match app.cmd {
        Some(ref method) => {
            let resp = client::perform_method(&app, method)?;
            handle_response(resp)
        }
        None => {
            let url = app.url.take().unwrap();
            let has_data = app.parameters.iter().any(|p| p.is_data());

            let method = if has_data {
                reqwest::Method::POST
            } else {
                reqwest::Method::GET
            };

            let resp = client::perform(&app, method, &url, &app.parameters)?;

            handle_response(resp)
        }
    }
}