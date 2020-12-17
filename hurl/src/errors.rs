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
    This mainly serves the purpose of getting the nice error message in some contexts
    which do a debug print by default where there is a lack of control

    The following crates provide some interesting features around errors:
        - error-chain
        - failure
        - context-attribute
        - err-derive
        - snafu
        - fehler
        - anyhow
        - thiserror

    Last piece is to provide implementations of the From trait for errors
    from other libraries that will be wrapped in the custom enum

    There are four that are relevant:
        - reqwest::Error
        - serde_json::error::Error
        - std::io::Error (dealing with file system errors)
        - reqwest::UrlError (URL parsing)
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
            Error::ClientSerialization => {
                write!(f, "Serializing the request/response failed")
            }
            Error::ClientTimeout => {
                write!(f, "Timeout during request")
            }
            Error::ClientWithStatus(status) => {
                write!(f, "Got status code: {}", status)
            }
            Error::ClientOther => {
                write!(f, "Unknown client error")
            }
            Error::SerdeJson(c) => {
                write!(f, "JSON error: {:?}", c)
            }
            Error::IO(k) => {
                write!(f, "IO Error: {:?}", k)
            }
            Error::UrlParseError(e) => {
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

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::UrlParseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    #[inline]
    fn from(err: reqwest::Error) -> Error {
        if err.is_serialization() {
            return Error::ClientSerialization;
        }

        if err.is_timeout() {
            return Error::ClientTimeout;
        }

        if let Some(s) = err.status() {
            return Error::ClientWithStatus(s);
        }

        Error::ClientOther
    }
}

impl From<serde_json::error::Error> for Error {
    #[inline]
    fn from(err: serde_json::error::Error) -> Error {
        Error::SerdeJson(err.classify())
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(err: std::io::Error) -> Error {
        Error::IO(err.kind())
    }
}

impl From<reqwest::UrlError> for Error {
    #[inline]
    fn from(err: reqwest::UrlError) -> Error {
        Error::UrlParseError(err)
    }
}