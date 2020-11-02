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

    FROM AND INTO

    Rust usually does not implicit convert one type to another,
    but there is a mechanism for telling Rust how to convert between types which you can opt in to

    You must explicitly implement one of these traits (From or Into) to be able to take advantage of some automatic type conversions

    One place that uses From is the ? operator for early returning the Err variant of a Result.
    That is if the error that would be returned is type X and
    the expected return type is Y
    then you can still use the ? operator if Y implements From<X>

    The AppError type will implement From<X> for a couple different values of X
    so that the ? operator works without having to explicitly convert errors

    So From<diesel::result::Error> for AppError means that you will be given an instance of diesel::result::Error
    and are expected to return an instance of AppError

    DATABASE ERRORS

    We convert the DatabaseError(UniqueViolation, _) to our RecordAlreadyExists variant
    as we will only get unique constraint violations
    when we try to insert a record that already exists based on what we have defined to be unique.

    Specifically, we have set a unique constraint on username
    so trying to insert two users with the same username
    will result in this RecordAlreadyExists error being created

    The second case is when we try to get a record from the database that does not exist.
    Diesel will return a NotFound error which we just turn into our variant with basically the same name

    Finally, the catch all case in the match statement means Diesel encountered an error other than these two
    and the only thing we know how to do is call it a DatabaseError

    ERRORS AS RESPONSES

    Main advantage of creating our own error type is that we define how to turn an instance of AppError into an HTTP response

    The struct ErrorResponse will represent a JSON error response.

    Actix web defines a trait ResponseError
    which allows you to specify how the type inside a Err variant of a Result gets turned into a response

    RESPONSE ERROR TRAIT FROM ACTIX

    The trait (actix_web::ResponseError) is why we implemented Display for our AppError.
    ResponseError has the trait bound Debug + Display
    which means that in order to implement ResponseError for your type,
    your type must also implement Debug and Display.

    The trait requires error_response to be implemented
    which we do by matching on our error and
    setting useful response codes to the cases we care about and 500 otherwise,
    and then using the Display formatting to create an error message to return as JSON.

    The trait also has a method render_response
    which has a default implementation,
    but the default overrides the content type and data which is not what we want

 *
***/

use actix_web::error::BlockingError;
use actix_web::web::HttpResponse;
use diesel::result::{
    DatabaseErrorKind::UniqueViolation,
    Error::{DatabaseError, NotFound}
};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    RecordAlreadyExists,
    RecordNotFound,
    DatabaseError(diesel::result::Error),
    OperationCanceled,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    err: String,
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

impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            DatabaseError(UniqueViolation, _) => AppError::RecordAlreadyExists,
            NotFound => AppError::RecordNotFound,
            _ => AppError::DatabaseError(e),
        }
    }
}

impl From<BlockingError<AppError>> for AppError {
    fn from(e: BlockingError<AppError>) -> Self {
        match e {
            BlockingError::Error(inner) => inner,
            BlockingError::Canceled => AppError::OperationCanceled,
        }
    }
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let err = format!("{}", self);
        let mut builder = match self {
            AppError::RecordAlreadyExists => HttpResponse::BadRequest(),
            AppError::RecordNotFound => HttpResponse::NotFound(),
            _ => HttpResponse::InternalServerError(),
        };
        builder.json(ErrorResponse { err })
    }

    fn render_response(&self) -> HttpResponse {
        self.error_response()
    }
}