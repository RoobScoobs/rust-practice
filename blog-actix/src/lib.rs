/*** 
 *
    SETTING UP THE LIBRARY

    Working with Diesel requires some macros to be in scope
    which is why we have the macro_use attribute on the extern crate diesel item.

    We also want to derive the Serialize and Deserialize traits from Serde for working with JSON
    so we bring those macros in to scope.

    Diesel has a prelude which includes common types and functions
    which are almost always needed while working with the database

    r2d2 is a connection pooling library that Diesel provides an interface to

    STRUCT BLOG

    The run method takes a String representation of a database URL as input
    which we use to construct a pool of database connections

    The ConnectionManager type is generic over the underlying connection which we specify as SqliteConnection

    The SqliteConnection type is imported as part of the Diesel prelude

    The type of the state is a connection pool

    As the closure that gets passed to HttpServer::new is a factory
    we have to clone our pool so that each worker will have access to the same shared pool 

    The Pool type is just an Arc around a struct that manages connections
    so calling clone on the pool is the same as calling clone on an Arc

    FnOnce TRAIT

    We are only guaranteed that it is okay to call the function once
    *
***/

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

mod errors;
mod models;
mod routes;
mod schema;

pub struct Blog {
    port: u16,
}

impl Blog {
    pub fn new(port: u16) -> Self {
        Blog { port }
    }

    pub fn run(&self, database_url: String) -> std::io::Result<()> {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        println!("Starting http server: 127.0.0.1:{}", self.port);

        HttpServer::new(move || {
            App::new()
                .data(pool.clone())
                .wrap(middleware::Logger::default())
                .configure(routes::users::configure)
                .configure(routes::posts::configure)
                .configure(routes::comments::configure)
        })
        .bind(("127.0.0.1", self.port))?
        .run()
    }
}