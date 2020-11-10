/*** 
 *
   MODELS MODULE

   To define the Rust representation of our data model as represented by our database

   As we're working with the database, we need to pull in the Diesel prelude and
   also bring in the users item from the autogenerated schema

   ALIAS TYPE

   We'll define our own Result type
   which will be an alias for Result in the standard library
   with the error type fixed as our AppError type

   This way we can just return Result<T> and not have to write AppError everywhere

   USERS STRUCT

   Our users have ids which are i32 because that maps to the database integer type
   and a username which is a String because the database column is a VARCHAR

   DERIVABLE TRAITS

   Queryable is a trait that indicates this struct can be constructed from a database query

   From the docs:
      Diesel represents the return type of a query as a tuple.
      The purpose of this trait is to convert from a tuple of Rust values that have been deserialized into your struct

   Identifiable is a trait that indicates that this struct represents a single row in a database table.
   
   It assumes a primary key named id
   but you can configure the derive attribute
   if you want to change the name of the primary key

   CREATE USER

   create_user function takes a SqliteConnection and a username string
   and returns either a User or an error

   Sqlite does not support getting the id of a just inserted row as part of the insert statement.
   Instead we have to do another query to actually get the data back out to build a User struct

   The connection type supports a method transaction which takes a closure.
   The closure must return a Result.

   If the Result is the error variant
   then the transaction is rolled back and
   the error result is returned

   If instead we have the Ok variant
   then the transaction is committed and
   the successful result is returned.

   FIRST DIESEL STATEMENT

   The imported users item from the schema has an item for each column on the table
   as well as an item for the table as a whole, i.e. users::table

   Each database column has a unique type so you catch at compile time
   if you tried to insert the wrong type into a column rather than at runtime.

   The execute method on the query builder returns a Result with the error being the Diesel specific error type.
   We can use the ? operator here and return a Result with our AppError error type
   because of our From implementation in our errors module

   SECOND STATEMENT INSIDE THE TRANSANCTION

   We're fetching a single user from the database.
   We create a query where we order by descending id to get the most recently inserted row

   As this is inside a transaction this select will execute logically after the insertion
   and therefore this will be the user we just inserted

   We specify with the select method what columns we want in a tuple,
   in this case id and username

   The first method tells Diesel to do a LIMIT 1 and expect a single result

   This therefore returns a Result with the successful variant being the resulting record or a not found error

   The return type of our function is what gives Diesel enough information to convert the tuple
   that is returned into our User struct in addition to the fact that we derived Queryable.

   Finally we call map_err, which is a method on Result that helps transform the error variant
   with the supplied funciton and returns the success variant as is.
   
   We cannot use the ? operator here to return our Result
   as we have to explicitly convert the Diesel error type to our AppError to match the function signature

   We can pass the function Into::into to map_err and
   this uses the function signature to determine what to transform the error into.
   
   Furthermore, we can use Into here because we implemented From and Into gets implemented automatically.

   FETCHING A USER

   Two ways to identify a user: by id and by username

   The UserKey enum is either an ID which holds an i32 for looking up by the primary key or
   a Username which holds a reference to a string

   GENERIC LIFETIME FOR A TYPE
   
   Lifetimes of references in Rust are checked by the compiler to ensure
   that the data referenced outlives the reference to it

   Here our type UserKey<'a> specifies that it has one generic lifetime parameter named 'a

   We need to specify this generic parameter
   so that we can give a definite lifetime to the string reference inside our Username variant

   In other words, we need to ensure that any reference to a UserKey
   cannot outlive the reference to the string slice it contains

   Any composite type with a reference inside must declare a lifetime on that reference
   otherwise there is no way for the compiler to make any guarantees about the liveness of the underlying data

   FUNCTION WHICH GETS THE USER find_user

   We match on our key to decide which query to run.
   
   If we have a username,
   then we do a filter by username equal to the value passed in.
   
   If we have an id,
   then we can use the special find function
   which attempts to find a single record based on the primary key for that table

   POST MODEL

   Our database schema is directly translated to Rust types where each field is a column of the appropriate type

   Note that the not null constraints we specified in our migration on each column is why we use types directly
   rather than wrapping them in Option

   ASSOCIATION

   The concept of an association in Diesel is always from child to parent

   Declaring the association between two records requires the belongs_to attribute on the child
   and specifies the name of the struct that represents the parent

   The belongs_to attribute accepts a foreign_key argument
   if the relevant foreign key is different from table_name_id
   
   Both the parent and child must implement the Identifiable trait

   In addition to the belongs_to annotation,
   the struct also needs to derive Associations
   
   Deriving this trait uses the information in belongs_to to generate the relevant code to make joins possible

 *
***/

use crate::errors::AppError;
use crate::schema::{users, posts};
use diesel::prelude::*;

type Result<T> = std::result::Result<T, AppError>;

#[derive(Queryable, Identifiable, Serialize, Debug, PartialEq)]
pub struct User {
   pub id: i32,
   pub username: String,
}

#[derive(Queryable, Associations, Identifiable, Serialize, Debug)]
#[belongs_to(User)]
pub struct Post {
   pub id: i32,
   pub user_id: i32,
   pub title: String,
   pub body: String,
   pub published: bool,
}

pub enum UserKey<'a> {
   Username(&'a str),
   ID(i32),
}

pub fn create_user(conn: &SqliteConnection, username: &str) -> Result<User> {
   conn.transaction(|| {
      diesel::insert_into(users::table)
         .values((users::username.eq(username),))
         .execute(conn)?;

      users::table
         .order(users::id.desc())
         .select((users::id, users::username))
         .first(conn)
         .map_err(Into::into)
   })
}

pub fn find_user<'a>(conn: &SqliteConnection, key: UserKey<'a>) -> Result<User> {
   match key {
      UserKey::Username(name) => users::table
         .filter(users::username.eq(name))
         .select((users::id, users::username))
         .first::<User>(conn)
         .map_err(AppError::from),

      UserKey::ID(id) => users::table
         .find(id)
         .select((users::id, users::username))
         .first::<User>(conn)
         .map_err(Into::into),
   }
}