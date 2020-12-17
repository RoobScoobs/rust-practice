/***
 * 
 * 
 * 
    SESSIONS MODULE

    A common pattern that comes up when testing APIs on the command line
    is to make multiple requests that depend on one another

    The most common dependency comes from authentication where a user logs in
    and then wants to perform some actions

    A session is the common terminology for a set of activity between logging in and logging out

    DEFINING THE SESSION

    Deriving Serialize and Deserialize in order to read and write the struct a file
    The path, name and host are used uniquely to describe the file system location for the session

    Constructing a new session is accomplished by using a helper method to get a path
    and then storing the data given inside a new session struct
    and using default values for the rest

    The syntax ..Default::default() inside a struct literal is known as the struct update syntax

    It means to use the values from a default constructed instance of session to fill in any fields
    which are not provided above

    As the struct is deriving Default it is one way to create a full Session struct with default values
    If other_session was a variable with the type Session,
    then it's possible to write ..other_session to use the values from other_session to fill in the missing values

    LOADING A SESSION FROM A FILE

    Use the path helper function to get the path,
    then perform the standard sequence of opening a file, putting a reader around the file handle,
    and then using serde to deserialize a type from the data in the file

    Here serde knows what to turn the data in the file into because of the return type of the function

    get_or_create is more or less a convenience method that puts together load and new

    GETTING A CONSISTENT PATH FOR STORING THE SESSION FILE

    Using another helper to get the directory
    which is combined with the result of make_safe_pathname applied to the name of the session
    to get a full path to the file where the session is to be stored

    The dir helper starts with getting a directory based on the app configuration or using a default,
    and then adds a path based on the host

    SAVING A SESSION

    The session needs to be able to save itself to disk, hence the save method

    Start by ensuring the directory where the file to store exists

    The function create_all_dir is effectively mkdir -p which creates all intermediate directories as needed

    Then the file is opened at the specified path and the data structure is serialized to disk as JSON

    The OpenOptions builder is used to specify exactly how to open the file
    
    Aim is to have the file on disk to represent the current struct
    that's being written in both cases where the file does and does not exist

    The create function is needed for the does not exist case,
    and the truncate function is needed to properly handle all cases when the file does exist

    UPDATING THE SESSION FROM REQUEST PARAMETERS

    The Parameter::Header is the only parameter to care about
    because the app only supports headers as a parameter that is stored in the session

    Headers are stored in the headers field on the session, but a few keys are excluded
    as persisting them across request is a no-no

    Next, if auth data is available, the session needs to be updated

    The add_to_request method is as the name implies adding the session to the request

    It starts by adding headers to the request, if there are any
    Further, if there are cookies, 
    turn them into the expected format for the cookie header and add that to the request

    The format of the cookie header is given by the HTTP specification
    and the key for the cookie header is provided by reqwest as the constant COOKIE

    The only part of the response that is to be absorbed into the session is the cookies,
    so update_with_response comes in to update the session accordingly

    The make_safe_pathname helps turn a string into something that is safe for storing on the file system
    This is just one example of a scheme that works but can be something else
***/

use crate::app::{App, Parameter};
use crate::directories::DIRECTORIES;
use crate::errors::HurlResult;
use reqwest::header::COOKIE;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Session {
    path: PathBuf,
    name: String,
    host: String,
    auth: Option<String>,
    token: Option<String>,
    headers: HashMap<String, String>,
    cookies: Vec<(String, String)>,
}

impl Session {
    pub fn new(app: &App, name: String, host: String) -> Self {
        let path = Session::path(app, &name, &host);

        Session {
            path,
            name,
            host,
            ..Default::default()
        }
    }

    pub fn load(app: &App, name: &str, host: &str) -> HurlResult<Self> {
        let path = Session::path(app, name, host);
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        serde_json::from_reader(reader).map_err(|e| e.into())
    }

    pub fn get_or_create(app: &App, name: String, host: String) -> Self {
        match Session::load(app, &name, &host) {
            Ok(session) => session,
            Err(_) => Session::new(app, name, host),
        }
    }

    fn path(app: &App, name: &str, host: &str) -> PathBuf {
        let mut session_dir = Session::dir(app, host);
        let mut filename = make_safe_pathname(name);

        filename.push_str(".json");
        session_dir.push(filename);
        session_dir
    }

    fn dir(app: &App, host: &str) -> PathBuf {
        let mut session_dir = app
            .session_dir
            .as_ref()
            .cloned()
            .filter(|session_dir| session_dir.is_dir())
            .unwrap_or_else(|| DIRECTORIES.config().join("sessions"));
        
        session_dir.push(make_safe_pathname(host));
        session_dir
    }

    pub fn save(&self, app: &App) -> HurlResult<()> {
        let dir = Session::dir(app, &self.host);
        create_dir_all(dir)?;

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        let writer = BufWriter::new(file);

        serde_json::to_writer(writer, &self).map_err(|e| e.into())
    }

    pub fn update_with_parameters(&mut self, parameters: &Vec<Parameter>) {
        for parameter in parameters.iter() {
            match parameter {
                Parameter::Header { key, value } => {
                    let lower_key = key.to_ascii_lowercase();

                    if lower_key.starts_with("content-") || lower_key.starts_with("if-") {
                        continue;
                    }

                    self.headers.insert(key.clone(), value.clone());
                }
                _ => {}
            }
        }
    }

    pub fn update_auth(&mut self, auth: &Option<String>, token: &Option<String>) {
        if auth.is_some() {
            self.auth = auth.clone();
        }

        if token.is_some() {
            self.token = token.clone();
        }
    }

    pub fn add_to_request(&self, mut builder: RequestBuilder) -> RequestBuilder {
        for (key, value) in self.headers.iter() {
            builder = builder.header(key, value);
        }
        let cookies = self
            .cookies
            .iter()
            .map(|(name, value)| format!("{}={}", name, value))
            .collect::<Vec<String>>()
            .join("; ");

        if cookies.is_empty() {
            return builder;
        }

        builder.header(COOKIE, cookies)
    }

    pub fn update_with_response(&mut self, resp: &reqwest::Response) {
        for cookie in resp.cookies() {
            self.cookies
                .push((cookie.name().to_owned(), cookie.value().to_owned()));
        }
    }
}


pub fn make_safe_pathname(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());

    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | ' ' => buf.push(c),
            _ => buf.push('_'),
        }
    }

    buf
}