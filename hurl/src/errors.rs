/***
 * 
 * 
 * 
    ERROR HANDLING

    Might be tempting to write code using unwrap and panic! or returning results with strings for errors
    but creating a simple error enum along with a type alias for a particular result is a tiny amount of work
    thus it's worthwhile to do

    Importing std::fmt to make our Display implementation easier to write
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