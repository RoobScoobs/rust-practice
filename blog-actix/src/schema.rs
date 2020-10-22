/* 
    
    Diesel uses to schema.rs to understand the state of the database

*/

table! {
    users (id) {
        id -> Integer,
        username -> Text,
    }
}
