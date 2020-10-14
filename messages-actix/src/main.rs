/***
 *
    Crate names are allowed to contain hyphens and underscores,
    but identifiers in Rust are not allowed to contain hyphens.

    Therefore, if you use a crate name with an underscore the compiler knows to look for a crate
    with a hyphen if one with an underscore cannot be found.

    AGGREGATE DATA TYPES

    The primary mechanisms for creating aggregate data types in Rust are enums and structs.

    RESULT TYPE

    The Result type is one of the primary error handling primitives that Rust provides.

    Within the std::io module there is a type definition:
        - type Result<T> = Result<T, std::io::Error>;

    The syntax for the type definition above just says to create an alias std::io::Result<T> 
    which uses the standard Result type with the second type parameter (which represents the type inside the error variant)
    fixed to the one defined in the std::io module.

    Therefore, std::io::Result<()> is the same as Result<(), std::io::Error>.

    () - known as the empty tuple
    This is commonly used as a marker where you don’t actually care about the value but you need some placeholder type.
    
***/

use messages_actix::MessageApp;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix-web=info");
    env_logger::init();
    let app = MessageApp::new(8080);
    app.run()
}
