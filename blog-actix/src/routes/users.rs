/*** 
 *
    ROUTES FOR A USER

    Note that our convert function in the routes module was not public but we are using it here

    Private items are visible to the module they are defined in as well as all descendants

    CREATE A USER

    Our first handler is going to take a username as input and attempt to create a user.
    We'll then create a struct to represent the input data: UserInput

    As we derive Deserialize we will be able to accept this type as a JSON post body

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