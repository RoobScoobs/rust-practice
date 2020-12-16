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
    cookies: Vec<(String, String),
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

    pub fn save(&self, app: &App) -> HurlResult<()> {}
}