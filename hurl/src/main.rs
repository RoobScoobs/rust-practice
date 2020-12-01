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
***/

use structopt::StructOpt;
use heck::TitleCase;
use log::trace;

mod app;
mod client;
mod errors;

use errors::HurlResult;

type OrderedJson = std::collections::BTreeMap<String, serde_json::Value>;