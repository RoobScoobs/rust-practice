/*** 
 *
    ROUTES MODULE
    
    Defining the handlers that will make up the function that get called by the framework in response to web requests

    SUBMODULES

    Declaring the users submodule

    You can declare an item as public with pub
    then it can be accessed externally from any module that can access all of the item’s parent modules

    If you want to restrict the visibility of an item to a specific scope
    then you can use one of pub(in path), pub(crate), pub(super), or pub(self), where path is a given module path

    WHAT SUPER MEANS

    The module above routes is our root module at src/lib.rs.
    
    We want that module to be able to refer to the users module.
    However we do not want, say our models module, to be able to refer to the users module.
    So we restrict the visibility of the users module to only the module one step up in the hierarchy

    GENERIC CONVERT FUNCTION 
    
    This function takes some generic result and returns another result with fixed types

    Success variant: a successful HTTP response with the data serialized as JSON

    Error variant: AppError type that can be returned from a handler
                   and will result in a response with the status code and JSON error message

    Placed trait bounds on the generic parameters to specify that we can only accept input arguments
    if the success variant is a type that can be serialized to JSON, i.e. T: serde::Serialize
    and we can get an AppError from the error variant, i.e. AppError: From<E>

    In terms of implementation we take the result and
    call map which operates only on the success variant and builds a response

    The json method on the response builder just requires
    that the argument passed can be serialized with Serde

    Then we chain the call with the invocation of map_err
    which operates only on the error variant

 *
***/

use crate::errors::AppError;
use actix_web::HttpResponse;

pub(super) mod users;
pub(super) mod posts;

fn convert<T, E>(res: Result<T,E>) -> Result<HttpResponse, AppError>
where
   T: serde::Serialize,
   AppError: From<E>,
{
   res.map(|d| HttpResponse::Ok().json(d))
      .map_err(Into::into)
}