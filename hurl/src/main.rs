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

    THE RESPONSE HANDLER

    Expect a response as input, which in this case is just the Response type from the reqwest crate,
    and the HurlResult type is returned

    The purpose of the hurl tool is to print useful information to standard output
    and therefore “handling” the response means doing that printing

    First will build up a string based on the metadata about the response,
    then will print the rest of the body

    Useful pieces of data include the status code of the response
    as well as the version of HTTP that was used

    The string s is going to be the buffer that is used for building up the output

    format! can be used to create a String based on the version stored in the response,
    the status code, and the description of that status

    Example results:
        "HTTP/1.1 200 OK", or
        "HTTP/1.1 403 Forbidden"

    Want to use the Debug output - {:?} - of resp.version()

    SHOW THE HEADERS FROM THE RESPONSE

    The response type has a headers function
    which returns a reference to a Header type that gives us access to the headers
    
    Able to turn the headers into an iterator by calling the iter
    so each key/value pair can be processed
    
    Again using the format! macro to construct a string for each header

    The to_title_case method is available because we imported the TitleCase trait from heck
    which allows for the transformation of the raw key in the header map into a consistent style

    Expectation is that the value will have a string representation,
    but in case that fails a “BAD HEADER VALUE” string is used as a replacement

    Do not want to fail the entire response printing
    if one header value cannot be printed correctly
    thus unwrap_or provides a way to handle this case
    
    The reqwest crate does not treat content length as a normal header value
    and instead provides a content_length function on the response type to get this value

    content.length() returns an Option, so have to deal with computing a value if the function returns None
    and that value will be set to the length of the text of the response ergo: resp.text()?.len()

    WHY DOES REQWEST BEHAVE IN THIS WAY?

    Main reason could be because of compression

    The content length of the response is not necessarily the same as the content length
    that is given in the response header because the actually response body could be compressed

    After decompressing the body, it results in a different length

    The library returns None in this case to signal that if you want to compute an accurate content length,
    you have to do it yourself

    PUTTING THE HEADERS AND STATUS STRING TOGETHER

    The headers were put into a vector in order to sort by the name of the header
    Then by using the join method on the string slices
    able to turn the list of headers into a newline separated string

    As join is not a method on Vec, have to use &headers[..] to get a reference to a slice of type &[String]
    Then turn the output of that function from String to &str with the extra & at the beginning

    PRINTING OUT THE BODY OF THE RESPONSE

    Making use of the raw text of the response in the variable result
    and aim here is to pretty print the JSON results if the result is JSON

    The actual representation of the JSON will be ordered by the keys if it is correctly parsed
    This is because of the type alias for OrderedJson telling serde that it should use a BTreeMap
    as the container for the top level JSON object

    ADDING CONFIGURATION TO THE APP

    Calling process_config_file after parsing and validating the command line arguments
    will get the configuration data incorporated into the app

    ADDING SESSION

    Before matching on the cmd argument, try to obtain a session
    The session variable here has type Option<Session>

    There will be a type of session field on the app which is optional
    and if it exists then that will be turned into a Session

    Adding the app annd session to the handle_response function as the response
    might update the session

    Inside the function, before returning, include some logic to check
    whether the session is not read only and the session exists, 
    in which case the session is updated from the response and saved

    While changing the handle_response, also want to change the calls to perform_method
    and perform as they will also take in the session
    
    Passing a mutable reference to the perform functions
    because the session is needed to possibly fill in data in the request
    and the session will be filled in with other data provided as part of the request as parameters

    ADDING SYNTAX HIGHLIGHTING
    
    The syntect dependency is added to for a bit of polish to the application

    A SyntaxSet is a set of grammatical syntaxes which can be highlighted
    almost thinking of it like a way to go from raw text to structured text

    A theme is a description of what colors to apply based on the annotations in that structured text

    The solarized dark theme is included by default by the syntect crate

    The response is to be highlighted, so need to add the syntax set and theme to handle_response
    Instead of calling println!, going to call highlight_string which will still print the string but with colors

    However still need to call println!("") to add a newline between the header content and the body

    If the body is parsed as JSON - the Ok branch in the match statement - then replace the print call again with highlight_string

    The syntect crate provides an incredible amount of intricate customizations and details for syntax highlighting,
    but it also provides some easy convenience methods when there's no need to get too deep into the details

    There are two different syntaxes used "JSON" and "HTTP"
    and those are passed as the syntax argument to the highlight_string function
    so the right syntax can easily be looked up from the syntax set

    Using this syntax and theme to construct a HighlightLines object
    which is used to generate text in colored regions for printing

    The as_24_bit_terminal escaped method is a utility for turning the colored regions
    into escape sequences for use in terminal output

    The LinesWithEndings will add newlines if they don't exist so
    print! instead of println! can be used and still get newlines at the end of the lines

    Finally, print out the terminal reset character which ends all highlighting
    and puts the user back into normal shell mode,
    so that the highlighting isn't leaked onto later shell commands
***/

use heck::TitleCase;
use log::trace;
use structopt::StructOpt;
use syntect::highlighting::Theme;
use syntect::parsing::SyntaxSet;

mod app;
mod client;
mod config;
mod directories;
mod errors;
mod session;
mod syntax;

use errors::HurlResult;

type OrderedJson = std::collections::BTreeMap<String, serde_json::Value>;

fn main() -> HurlResult<()> {
    let mut app = app::App::from_args();
    app.validate()?;
    app.process_config_file();

    if let Some(level) = app.log_level() {
        std::env::set_var("RUST_LOG", format!("hurl={}", level));
        pretty_env_logger::init();
    }

    let (ss, ts) = syntax::build()?;
    let theme = &ts.themes["Solarized (dark)"];

    let mut session = app
        .session
        .as_ref()
        .map(|name| session::Session::get_or_create(&app, name.clone(), app.host()));

    match app.cmd {
        Some(ref method) => {
            let resp = client::perform_method(&app, method, &mut session)?;
            handle_response(&app, &ss, theme, resp, &mut session)
        }
        None => {
            let url = app.url.take().unwrap();
            let has_data = app.parameters.iter().any(|p| p.is_data());

            let method = if has_data {
                reqwest::Method::POST
            } else {
                reqwest::Method::GET
            };

            let resp = client::perform(&app, method, &mut session, &url, &app.parameters)?;

            handle_response(&app, &ss, theme, resp, &mut session)
        }
    }
}

fn handle_response(
    app: &app::App,
    ss: &SyntaxSet,
    theme: &Theme,
    mut resp: reqwest::Response,
    session: &mut Option<session::Session>
) -> HurlResult<()> {
    let status = resp.status();

    let mut s = format!(
        "{:?} {} {}\n",
        resp.version(),
        status.as_u16(),
        status.canonical_reason().unwrap_or("Unknown")
    );

    let mut headers = Vec::new();

    for(key, value) in resp.headers().iter() {
        let nice_key = key.as_str().to_title_case().replace(' ', "-");

        headers.push(format!(
            "{}: {}",
            nice_key,
            value.to_str().unwrap_or("BAD HEADER VALUE")
        ));
    }

    let result = resp.text()?;

    let content_length = match resp.content_length() {
        Some(len) => len,
        None => result.len() as u64,
    };

    headers.push(format!(
        "Content-length: {}",
        content_length
    ));

    headers.sort();
    s.push_str(&(&headers[..]).join("\n"));
    highlight_string(ss, theme, "HTTP", &s);

    println!("");

    let result_json: serde_json::Result<OrderedJson> = serde_json::from_str(&result);

    match result_json {
        Ok(result_value) => {
            let result_str = serde_json::to_string_pretty(&result_value)?;

            highlight_string(ss, theme, "JSON", &result_str);
        }
        Err(e) => {
            trace!("Failed to parse result to JSON: {}", e);
            println!("{}", result);
        }
    }

    if !app.read_only {
        if let Some(s) = session {
            s.update_with_response(&resp);
            s.save(app)?;
        }
    }

    Ok(())
}

fn highlight_string(ss: &SyntaxSet, theme: &Theme, syntax: &str, string: &str) {
    use syntect::easy::HighlightLines;
    use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

    let syn = ss
        .find_syntax_by_name(syntax)
        .expect(&format!("{} syntax should exist", syntax));

    let mut h = HighlightLines::new(syn, theme);

    for line in LinesWithEndings::from(string) {
        let regions = h.highlight(line, &ss);
        print!("{}", as_24_bit_terminal_escaped(&regions[..], false));
    }

    println!("\x1b[0m");
}