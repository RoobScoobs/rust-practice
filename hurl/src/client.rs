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
***/

use crate::app::{App, Method, Parameter};
use crate::errors::{Error, HurlResult};
use log::{info, debug, trace, log_enabled, self};
use reqwest::multipart::Form;
use reqwest::{Client, RequestBuilder, Response, Url};
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
    builder = handle_auth(builder, &app.path, &app.token)?;

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