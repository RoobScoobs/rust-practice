/***
 * 
 * 
 * 
    THE DIRECTORIES

    This module is responsible for doing the cross platform home directory lookup

    The first import is bringing in the lazy_static macro

    If building for MacOS can conditionally include an import by using the cfg attribute

    The Directories struct holds the default path to the configuration file

    METHODS ON THE DIRECTORIES TYPE

    The new method returns an Option because it's possible that directory to look for isn't found

    The config_op variable will only be defined once based on the OS compilation target

    After setting the home directory add "hurl" to the end of the path
    and place that path inside the Directories struct

    Also created the config method which turns the PathBuf into a Path by way of the Deref trait

    Finally, use the lazy_static macro to expose a static reference to a newly constructed Directories struct

    Using expect to crash if unable to get a path to the home directory
    This only occurs when a path cannot be constructed, is not about whether the config directory exists
    or whether the config file exists

***/

use lazy_static::lazy_static;
use std::path::{Path, PathBuf};

#[cfg(target_os = "macos")]
use std::env;

pub struct Directories {
    config: PathBuf,
}

impl Directories {
    fn new() -> Option<Directories> {
        #[cfg(target_os = "macos")]
        let config_op = env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
            .or_else(|| dirs::home_dir().map(|d| d.join("config")));

        #[cfg(not(target_os = "macos"))]
        let config_op = dirs::config_dir();

        let config = config_op.map(|d| d.join("hurl"))?;

        Some(Directories { config })
    }

    pub fn config(&self) -> &Path {
        &self.config
    }
}

lazy_static! {
    pub static ref DIRECTORIES: Directories =
        Directories::new().expect("Could not get home directory");
}