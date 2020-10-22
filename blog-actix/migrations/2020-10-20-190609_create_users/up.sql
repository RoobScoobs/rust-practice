/**
    USING THE DIESEL CLI

    diesel migration generate create_users

    This creates a directory migrations/YYYY-MM-DD-HHMMSS_create_users with two empty files
    one of which is this particular file created as up.sql

    This is where our users table SQL goes

    RUNNING MIGRATIONS WITH DIESEL CLI

    diesel migration run
**/

CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL
)