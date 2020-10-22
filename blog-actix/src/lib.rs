/*** 
 *
    SETTING UP THE LIBRARY

    Working with Diesel requires some macros to be in scope
    which is why we have the macro_use attribute on the extern crate diesel item.

    We also want to derive the Serialize and Deserialize traits from Serde for working with JSON
    so we bring those macros in to scope.

    Diesel has a prelude which includes common types and functions
    which are almost always needed while working with the database
    *
***/

#[macro_use]
extern create diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};