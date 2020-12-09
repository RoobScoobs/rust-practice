/***
 * 
 *
    THE MECHANISM FOR MAKING HTTP REQUESTS

    As seen how the client module is used in main.rs
    there are two entry points, perform and perform_method
    
    PERFORM_METHOD 
    
    The fn takes the Method struct
    and gets the relevant data from it so that the more general perform function can be called

    The method.into() call converts our Method struct into the reqwest::Method type
    so the code for that conversion relies on the implementation of the From trait
    which provides the reciprocal Into trait

    PERFORM

    The first step is to create a Client which is imported from reqwest,
    parse the url into a something useful,
    and then further validate the parameters based on whether or not it is a multipart request

    Using a helper function -- p.is_form_file() which is defined on Parameter -- to determine whether this is a multipart request

    The Client type has a request method
    which returns a builder given a HTTP method and a URL

    Use some helpers (handle_parameters and handle_autho) to modify the builder with the various configuration details

    Finally, send the request with builder.send().map_err(From::from)

    However there are two cases, one where logging is enabled at at least the Info level and when it is not

    If there is logging, then the time facilities in the standard library is used to compute the time required to get the response back
    If we there isn't logging, then just send the request
    
    The map_err(From::from) bit is so that
    if an error is returned from reqwest it can be turned into a custom error type

    PARSE HELPER

    The parse function will take in the raw URL string
    and turn it into the Url type exposed by reqwest

    There are two ways to call localhost URLs:
        - If using the default port of 80 on localhost then the URL as :/some_path can be specified where some_path is optional
            > In that case the leading colon is stripped off
              and the rest of the given URL is interpolated into a string which explicitly mentions localhost
        - If something is placed after the colon which is not a slash
          then that is interpreted to mean that the localhost port will be specified along with the URL
          so the raw URL is just appended to localhost
            > In other words, if a request wants to be made to localhost:8080 can simply use :8080

    If neither of these two scenarios applies, then the given string is parsed directly
    
    If that succeeds then it can just be returned; otherwise, a scheme is added to the given URL

    The reqwest Url::parse function requires a scheme so just using example.com would fail to parse

    The App struct has a secure flag for whether to use https by default,
    so can switch on that value to decide which scheme to try

    HANDLE PARAMETERS HELPER

    The function takes a RequestBuilder and some parameter data and returns a builder or an error

    The mut annotation in front of the builder means
    to make the builder argument work like a mutable local variable

    This is set because the methods on RequestBuilder consume the receiver
    i.e. they take self as their first argument rather than &self or &mut self
    and return a new RequestBuilder

    A HashMap from strings to JSON values is declared
    and this will be the container for the data that will eventually add to the request

    If the request is a multipart form based on the argumented passed into the function,
    a Form object is created inside an Option for later use

    This form object is provided by reqwest for exactly this purpose of adding each part of a multipart form to a request

    Then start to loop over the parameters,
    and within the loop use a big match statement to handle each one individually

    THE MATCH STATEMENT FOR PARAMS

    The header type is straightforward: just call the header method to update the builder

    The Data type is slightly harder insofar as the key and value are inserted into the data map if a multipart form isn't being built
    otherwise add the key and value to the form using the methods on Form

    As the multipart variable is inside an Option, map is used to operate on the Form type inside

    Adding query string elements is also easy given the query method on the builder

    The RawJsonData type just uses serde to parse the string into a Value before inserting into the data hash map

    HANDLING FILES

    First, RawJsonDataFile parses the file into a JSON value using serde
    and inserts the result into the data map

    Provided by the standard library able to open a handle to a file
    and build a buffered reader around that handle

    The reader can then be passed directly to the from_reader function
    This automatically handles all of the complexity of ensuring the file is available,
    incrementally reading it, ensuring it is closed even if something goes wrong, etc

    DataFile reads a string from filename and inserts that string as a value directly in the hash map
        *Note*: the read_to_string method is not what you want in many cases dealing with file I/O in Rust, but here it's fine
    
    FormFile is simple here due to the file function provided by the Form type

    Calling unwrap on the multipart
    because the existence of a FormFile parameter is equivalent to is_multipart being true
    so it is known that multipart must not be None in this branch of the match statement

    Now that all the parameters are added to the hash map structure, they're passed along to the builder

    If the case is that there's a multipart form then use the multipart method on the builder,
    otherwise either the form or json methods are used depending on whether the is_form flag is passed

    Simply check to ensure that the data is not empty before tyring to serialize it as part of the request

    Finally sans any errors can successfully return the builder

    HANDLING AUTHENTICATION

    handle_auth is the helper responsible for modifying the builder to add the relevant data

    If there is no auth string or token then simply return the builder unchanged
    Otherwise use the basic_auth or bearer_auth methods on the builder to add the particular pieces of auth information

    The basic auth path makes use of another helper called parse_auth to get the username and password

    The app accpets the forms myUserName, myUserName:, and myUserName:myPassword as basic auth strings

    The first form without a colon assumes that the username was put in and user wants to type in their password

    Using the rpassword crate which provides the read_password_from_tty method asks the user to enter a password
    This captures that value and returns it as a string without echoing the user input

    The other two cases with colons mean that the user is giving a password and the app doesn't prompt the user to enter one
    In the first case - myUserName: - it's saying that no password will be provided

***/

use crate::app::{App, Method, Parameter};
use crate::errors::{Error, HurlResult};
use log::{info, debug, trace, log_enabled, self};
use reqwest::multipart::Form;
use reqwest::{Client, RequestBuilder, Response, Url};
use rpassword;
use serde_json::Value;
use std::collection::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

pub fn perform_method(
    app: &App,
    method: &Method
) -> HurlResult<Response> {
    let method_data = method.data();

    perform(
        app,
        method.into(),
        &method_data.url,
        &method_data.parameters
    )
}

pub fn perform(
    app: &App,
    method: reqwest::Method,
    raw_url: &str,
    parameters: &Vec<Parameter>
) -> HurlResult<Response> {
    let client = Client::new();
    let url = parse(app, raw_url)?;
    debug!("Parsed url: {}", url);

    let is_multipart = parameters.iter().any(|p| p.is_form_file());
    if is_multipart {
        trace!("Making multipart request because form file was given");
        if !app.form {
            return Err(Error::NotFormButHasFormFile);
        }
    }

    let mut builder = client.request(method, url);
    builder = handle_parameters(builder, app.form, is_multipart, parameters)?;
    builder = handle_auth(builder, &app.auth, &app.token)?;

    if log_enabled!(log::Level::Info) {
        let start = Instant::now();
        let result = builder.send().map_err(From:: from);
        let elasped = start.elapsed();
        info!("Elasped time: {:?}", elasped);
        result
    } else {
        builder.send().map_err(From::from)
    }
}

fn parse(app: &App, s: &str) -> Result<Url, reqwest::UrlError> {
    if s.starts_with(":/") {
        return Url::parse(&format!("http://localhost{}", &s[1..]));
    } else if s.starts_with(":") {
        return Url::parse(&format!("http://localhost{}", s))
    }

    match Url::parse(s) {
        Ok(url) => Ok(url),
        Err(_e) => {
            if app.secure {
                Url::parse(&format!("https://{}", s))
            } else {
                Url::parse(&format!("http://{}", s))
            }
        }
    }
}

fn handle_parameters(
    mut builder: RequestBuilder,
    is_form: bool,
    is_multipart: bool,
    parameters: &Vec<Parameter>
) -> HurlResult<RequestBuilder> {
    let mut data: HashMap<&String, Value> = HashMap::new();

    let mut multipart = if is_multipart {
        Some(Form::new())
    } else {
        None
    }

    for param in parameters.iter() {
        match param {
            Parameter::Header { key, value } => {
                trace!("Adding header: {}", key);
                builder = builder.header(key, value);
            }
            Parameter::Data { key, value } => {
                trace!("Adding data: {}", key);
                if multipart.is_none() {
                    data.insert(key, Value::String(value.to_owned()));
                } else {
                    multipart = multipart.map(|m| m.text(key.to_owned(), value.to_owned()));
                }
            }
            Parameter::Query { key, value } => {
                trace!("Adding query parameter: {}", key);
                builder = builder.query(&[(key, value)]);
            }
            Parameter::RawJsonData { key, value } => {
                trace!("Adding JSON data: {}", key);
                let v: Value = serde_json::from_str(value)?;
                data.insert(key, v);
            }
            Parameter::RawJsonDataFile { key, filename } => {
                trace!("Adding JSON data for key={} from file={}", key, filename);
                let file = File::open(filename)?;
                let reader = BufReader::new(file);
                let v: Value = serde_json::from_reader(reader)?;
                data.insert(key, v);
            }
            Parameter::DataFile { key, filename } => {
                trace!("Adding data from file={} for key={}", filename, key);
                let value = std::fs::read_to_string(filename)?;
                data.insert(key, Value::String(value));
            }
            Parameter::FormFile { key, filename } => {
                trace!("Adding file={} with key={}", filename, key);
                multipart = Some(
                    multipart
                        .unwrap()
                        .file(key.to_owned(), filename.to_owned())?,
                );
            }
        }
    }

    if let Some(m) = multipart {
        builder = builder.multipart(m);
    } else {
        if !data.is_empty() {
            if is_form {
                builder = builder.form(&data);
            } else {
                builder = builder.json(&data);
            }
        }
    }

    Ok(builder)
}

fn handle_auth(
    mut builder: RequestBuilder,
    auth: &Option<String>,
    token: &Option<String>
) -> HurlResult<RequestBuilder> {
    if let Some(auth_string) = auth {
        let (username, maybe_password) = parse_auth(&auth_string)?;
        trace!("Parsed basic authentication. Username={}", username);
        builder = builder.basic_auth(username, maybe_password);
    }

    if let Some(bearer) = token {
        trace!("Parsed bearer authentication. Token={}", bearer);
        builder = builder.bearer_auth(bearer);
    }

    Ok(builder)
}

fn parse_auth(s: &str) -> HurlResult<(String, Option<String>)> {
    if let Some(idx) = s.find(':') {
        let (username, password_with_colon) = s.split_at(idx);
        let password = password_with_colon.trim_start_matches(':');

        if password.is_empty() {
            return Ok((username.to_owned(), None));
        } else {
            return Ok((username.to_owned(), Some(password.to_owned())));
        }
    } else {
        let password = rpassword::read_password_from_tty(Some("Password: "))?;
        return Ok((s.to_owned(), Some(password)));
    }
}