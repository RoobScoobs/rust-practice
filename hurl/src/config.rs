/***
 * 
 * 
 * 
    THE CONFIG

    Module responsible for finding and loading the configuration file

    The use of Deserialize here is what allows the toml crate
    to use the serde machineary to turn the file into an instance of this struct

    config_file is a helper function that takes the App struct and returns a PathBuf to find a configuration file

    If the app has a valid file defined as its config field then use that,
    otherwise with the helper provided by the directories module
    can get a path to the default config directory

    By using unwrap_or_else can ensure that a PathBuf is always returned from the function

    read_config_file is another helper that attempts to read and parse the found file into the Config struct

    The Result returned by read_to_stringis turned into an Option by using ok()
    which is a common idiom when caring about the failure but not the specifics of the error

    The error variant just gets turned into a None thus able to use map on that option
    to operate only on the case when there's a string of data from a file

    Then the toml crate along with the use of serde enables turning that string into
    the expected data structure

    The use of unwrap here is for expedience
***/

use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

use crate::app::App;
use crate::directories::DIRECTORIES;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub verbose: Option<u8>,
    pub form: Option<bool>,
    pub auth: Option<String>,
    pub token: Option<String>,
    pub secure: Option<bool>,
}

pub fn config_file(app: &App) -> PathBuf {
    app.config
        .as_ref()
        .cloned()
        .filter(|config_path| config_path.is_file())
        .unwrap_or_else(|| DIRECTORIES.config().join("config"))
}

pub fn read_config_file(path: PathBuf) -> Option<Config> {
    fs::read_to_string(path).ok().map(|content| {
        let config: Config = toml::from_str(&content).unwrap();
        config
    })
}