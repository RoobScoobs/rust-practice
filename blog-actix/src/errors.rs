/*** 
 *
    ERRORS MODULE

    For working with various failure scenarios

    PURPOSE OF OUR IMPORTS

    To translate some actix_weberrors and some diesel errors to our own custom type

    To describe how our error type can be converted into an HTTP response

    Bring in the std::fmt module because we are going to implement trait from within that module for our error type

    ERROR ENUM TYPE

    In this context encountering an error means we enter one of a small number of states
    where we want to return an error code and a message instead of continuing to process the request

    The first two variants relate to errors returned from Diesel that we convert into easier to work with variants

    Also have a catch-all DatabaseError for other Diesel errors that we don’t specifically handle

    OperationCanceled is related to a actix_web error having to do with an async operation

    DEBUG AND DISPLAY TRAITS

    Automatically implemented the Debug trait with the derive attribute on our struct
    which allows us to format instances of our type with the debug string formatter: {:?}

    Implementing Display let’s us print our type with {}

    The macro write! is like println! except the first argument is a “Writer”
    and it returns a Result in the case of an error

    The &mut fmt::Formatter argument implements a trait that makes it a “Writer”
    so typically you just use write!(f, ...) and
    fill in the ... with whatever you want to represent your type when it is formatted using {}
 *
***/

use actix_web::error::BlockingError;
use actix_web::web::HttpResponse;
use diesel::result::{
    DatabaseErrorKind::UniqueViolation,
    Error::{DatabaseError, NotFound}
};
// use diesel::result::DatabaseErrorKind::UniqueViolation;
// use diesel::result::Error::{DatabaseError, NotFound};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    RecordAlreadyExists,
    RecordNotFound,
    DatabaseError(diesel::result::Error),
    OperationCanceled,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::RecordAlreadyExists => write!(f, "This record violates a unique constraint"),
            AppError::RecordNotFound => write!(f, "This record does not exist"),
            AppError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            AppError::OperationCanceled => write!(f, "The running operation was canceled"),
        }
    }
}