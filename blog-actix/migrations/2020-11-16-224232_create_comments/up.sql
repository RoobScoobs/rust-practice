/***
    diesel migration generate create_comments

    Here we create a table for comments, the difference being that
    comments will have foreign keys to both users and posts
***/

CREATE TABLE comments (
    id INTEGER PRIMARY  KEY NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id),
    post_id INTEGER NOT NULL REFERENCES posts (id),
    body TEXT NOT NULL
)