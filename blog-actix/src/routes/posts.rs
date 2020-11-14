/*** 
 *
 * 
 * 
    CREATING A POST

    The struct PostInput represents the JSON input data

    We only need to get the title and body of the Post
    as the rest of the information will be inferred from the URL or will take on a default value

    Our create_post function (in src/models.rs) takes a user struct as input rather than just a plain id,
    therefore we need to convert the id we take as input into a User before we can use it

    We do that so the error that results from a missing user happens first before we even try to create a post

    Then we use the and_then method on Result to continue on to creating a post only in the case where we actually found a user

    This way enables handling all of the different errors without having a mess of conditionals
***/

use crate::errors::AppError;
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use futures::Future;

#[derive(Debug, Serialize, Deserialize)]
struct PostInput {
    title: String,
    body: String,
}

fn add_post(
    user_id: web::Path<i32>,
    post: web::Json<PostInput>,
    pool: web::Data<Pool>
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        let key = models::UserKey::ID(user_id.into_inner());

        models::find_user(conn, key).and_then(|user| {
            let post = post.into_inner();
            let title = post.title;
            let body = post.body;

            models::create_post(conn, &user, title.as_str(), body.as_str())
        })
    })
    .then(convert)
}