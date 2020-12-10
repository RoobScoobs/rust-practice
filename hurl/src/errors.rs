/***
 * 
 * 
 * 
    ERROR HANDLING

    Might be tempting to write code using unwrap and panic! or returning results with strings for errors
    but creating a simple error enum along with a type alias for a particular result is a tiny amount of work
    thus it's worthwhile to do

    Importing std::fmt to make our Display implementation easier to write

    Then implement Debug directly to be the same as Display
***/

use std::fmt;

pub enum Error {
    ParameterMissingSeparator(String),
    MissingUrlAndCommand,
    NotFormButHasFormFile,
    ClientSerialization,
    ClientTimeout,
    ClientWithStatus(reqwest::StatusCode),
    ClientOther,
    SerdeJson(serde_json::error::Category),
    IO(std::io::ErrorKind),
    UrlParseError(reqwest::UrlError),
}

pub type HurlResult<T> = Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParameterMissingSeparator(s) => {
                write!(f, "Missing separator when parsing parameter: {}", s)
            }
            Error::MissingUrlAndCommand => {
                write!(f, "Must specify a url or a command")
            }
            Error::NotFormButHasFormFile => {
                write!(f, "Cannot have a form file 'key@filename' unless --form option is set" )
            }
            Error:ClientSerialization => {
                write!(f, "Serializing the request/response failed")
            }
            Error:ClientTimeout => {
                write!(f, "Timeout during request")
            }
            Error:ClientWithStatus(status) => {
                write!(f, "Got status code: {}", status)
            }
            Error:ClientTimeout => {
                write!(f, "Unknown client error")
            }
            Error:SerdeJson(c) => {
                write!(f, "JSON error: {:?}", c)
            }
            Error:IO(k) => {
                write!(f, "IO Error: {:?}", k)
            }
            Error:UrlParseError(e) => {
                write!(f, "URL Parsing Error: {}", e)
            }
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}