/**
    USING THE DIESEL CLI

    diesel migration generate create_posts

    Posts have a title and a body and a boolean specifying whether they are published or not.
    They also have a reference to a user which represents the author of the post

    After running diesel migration we'll see changes to the schema.rs file
    
    Peep the joinable! macro which says sets up the necessary Rust types to allow posts and users to be joined

    This is inferred from the database schema because of the foreign key specification in our migration
    i.e. REFERENCES users (id)

    Able to get the automatic relationship mapping in Diesel
    if the relationship is specified at the database level
**/

CREATE TABLE posts (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id),
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 0
)