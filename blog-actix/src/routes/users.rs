/*** 
 *
    ROUTES FOR A USER

    Note that our convert function in the routes module was not public but we are using it here

    Private items are visible to the module they are defined in as well as all descendants

    CREATE A USER

    Our first handler is going to take a username as input and attempt to create a user.
    We'll then create a struct to represent the input data: UserInput

    As we derive Deserialize we will be able to accept this type as a JSON post body

    We need the input data as the JSON body of the request
    and we need a handle to our database pool

    RETURN TYPE FUTURE

    Future<Item, Error> is an object that represents a computation
    which can be queried for a result or an error

    This is a standard approach to writing asynchronous code
    where you return immediately some value that represents a computation
    rather than doing the computation before returning

    Eventually the future resolves to a result or an error when the computation completes

    The syntax impl Future means that we are going to return some type that implements the Future trait

    As Future is a generic trait, we must fix the Item and Error types that are otherwise generic
    so that the return type is fully specified

    We want to return an HttpResponse in the successful case
    and our AppError in the other case

    INSIDE THE HANDLER

    We get a connection to the database out of the pool.

    We get the username from the input data. 

    Diesel is synchronous, it does not directly support futures for interacting with the database
    
    Therefore we use web::block
    which executes a blocking function on a thread pool and
    returns a future that resolves to the result of the function execution

    Finally, we can use our convert function to turn the result of the call to
    models::create_user into the response we desire

    FIND A USER

    find_user implements the lookup by username

    This will be a GET request so we expect the username to be a string in the path

    get_user implements the lookup by id

    This is basically the same as find_user except we expect an i32 in the path instead of a string
    and we create the other variant of the UserKey enum

    CONFIGURING THE ROUTES

    The signature of the configure function is specified by Actix web

    The only parameter is a mutable reference to a service configuration object

    Define 3 routes:
        - POST /users which calls create_user
        - GET /users/find/{name} which calls find_user
        - GET /users/{id} which calls get_user

    We use *to_async* to specify the handlers here 
    because our handlers return futures
    rather than *to* that we used before with synchronous handlers

    EXAMPLES TO TEST WITH CURL

    curl -H 'Content-Type: application/json' -X POST http://localhost:8998/users -d '{"username":"Ruben"}'
    curl -H 'Content-Type: application/json' http://localhost:8998/users/find/Ruben
    curl -H 'Content-Type: application/json' http://localhost:8998/users/1

 *
***/

use crate::errors::AppError;
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, HttpResponse};
use futures::Future;

#[derive(Debug, Serialize, Deserialize)]
struct UserInput {
    username: String,
}

fn create_user(
    item: web::Json<UserInput>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let username = item.into_inner().username;

        models::create_user(conn, username.as_str())
    })
    .then(convert)
}

fn find_user(
    name: web::Path<String>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let name = name.into_inner();
        let key = models::UserKey::Username(name.as_str());

        models::find_user(conn, key)
    })
    .then(convert)
}

fn get_user(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let id = user_id.into_inner();
        let key = models::UserKey::ID(id);

        models::find_user(conn, key)
    })
    .then(convert)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(web::post().to_async(create_user)))
        .service(web::resource("/users/find/{name}").route(web::get().to_async(find_user)))
        .service(web::resource("/users/{id}").route(web::get().to_async(get_user)));
}