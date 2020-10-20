/***
    Do this to ensure that usernames are unique in the system
    and that we can lookup users by their username quickly

    The UNIQUE keyword let’s us rely on the database for the enforcement of unique usernames.
***/

CREATE UNIQUE INDEX username_unique_idx ON users (username)