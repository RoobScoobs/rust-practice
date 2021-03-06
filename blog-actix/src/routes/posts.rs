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

    PUBLISHING A POST

    Simply need a post_id in the url path for processing a post publish

    FETCHING POSTS

    Can fetch posts either given a user_id or just fetch them all

    ROUTE CONFIGURATION

    The path /users/{id}/posts accepts both a POST and a GET request
    which route to our add_post and user_posts handlers, respectively
    Otherwise this is analogous to our configuration of the users routes

    TESTING THE POST API

    create a post: curl -s -H 'Content-Type: application/json' -X POST http://localhost:8998/users/1/posts -d 
        '{"title":"Ruben says hello", "body":"Hello to all"}'

    publish a post: curl -s -H 'Content-Type: application/json' -X POST http://localhost:8998/posts/1/publish

    list all posts: curl -s -H 'Content-Type: application/json' http://localhost:8998/posts

    view posts: curl -s -H 'Content-Type: application/json' http://localhost:8998/users/1/posts


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

fn publish_post(
    post_id: web::Path<i32>,
    pool: web::Data<Pool>
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        
        models::publish_post(conn, post_id.into_inner())
    })
    .then(convert)
}

fn user_posts(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();

        models::user_posts(conn, user_id.into_inner())
    })
    .then(convert)
}

fn all_posts(pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();

        models::all_posts(conn)
    })
    .then(convert)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users/{id}/posts")
            .route(web::post().to_async(add_post))
            .route(web::get().to_async(user_posts))
    )
    .service(web::resource("/posts").route(web::get().to_async(all_posts)))
    .service(web::resource("/posts/{id}/publish").route(web::post().to_async(publish_post)));
}