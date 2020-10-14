/***
 * 
    STRUCTS

    Structs have member data which can be of any type.

    Here we have one member named port of type u16.

    Each member field has its own privacy which is not exported by default. 
    Therefore,even though you can reference instances of type MessageApp outside of our library,
    you cannot directly access the port field.

    SEPARATION OF DATA AND FUNCTIONALITY

    We defined the data representation of our struct, but all methods associated with the type are defined elsewhere in what is known as an impl block.

    SELF keyword

    Inside an impl block Self has special meaning, it refers to the type on which we are defining the implementation.
    So we could have written the signature of new as:
        pub fn new(port: u16) -> MessageApp

    The name of new is not special, but has become convention as the name of the constructor function for types

    TAKING SELF AS A PARAMETER

    Class instance methods explicitly take self as their first parameter,
    and not taking self implies that the method is actually on the type rather than a particular instance.

    SPECIAL PARAMETER VALUES

    There are four special first parameter values: &self, self, &mut self, and mut self.
    All of the forms turn a function in a method on an instance of the type.
    This means that rather than being a function on the type which is called like MessageApp::new,
    we need to have constructed an instance of the type and then use dot syntax to call the method and set the first parameter

    &self
    The most common form 
    This means that our method takes an immutable reference to the instance invoking the method.
    We can read the data inside our type, but we cannot alter it.
    The calling code also maintains ownership so we are just borrowing the instance.

    self
    This form means that the method consumes self and therefore the instance that the method is being called on has its ownership moved into the method.
    This form comes usually when we are transforming a type into something else.

    &mut self
    This is the mutable version of the first form, self. The second most common thing you will encounter.
    Our method can read and write the data inside our type, but it does not own the value so this access is only temporary.

    mut self
    This method consumes self and self is mutable within the method.

    CLOSURES

    The basic syntax is to declare an argument list between pipes, ||,
    then possibly list the return value,
    followed by the function body between curly braces

    Type inference works on closures so we can usually omit types of the arguments and return values.
    
    If the keyword move comes before the argument list 
    then any variables from the environment that the closure uses are actually moved into the closure.
    This means the closure takes ownership of those variables rather than creating references.
    
    Without the move keyword, variables closed over are actually just references to the surrounding environment.

    Here we do not actually use any variables from the surrounding environment so this example would work without the move keyword.

    Our intent is for this closure to be entirely owned by the HttpServer and
    therefore the move signifies intent that the function should not have references to the environment in which it was created

    SYNTAX FOR WORKING WITH RESULT TYPE

    The Result type is quite special in Rust to the point of having special syntax 
    for the common pattern of returning an error early if one occurred
    or otherwise pulling the value out of the Ok case and continuing on

    The function bind returns a Result, by putting the ? after the call,
    we are saying that if the returned Result is the Err variant,
    then just return early with that value.

    Functionally equivalent to: 

    let result = HttpServer::new(move || {
        ...
    }).bind(("127.0.0.1", self.port));

    if result.is_err() {
    return Err(result.err().unwrap());
    }

    result.unwrap().workers(8).run()

    ATTRIBUTES
    A way of attaching metadata to a variety of things in the language.
    They can be attached to modules as a whole, structs, functions, and several other constructs.

    They can attach to the thing they are defined within using the syntax
    #![...] with a ! after the #

    Derive Attribute 
    The derive attribute is probably the most common attribute you will encounter.
    It allows you to implement traits for types without having to do any more work provided the type meets the requirements for the trait to be derived.
    
    It is possible to write custom derive logic so that your own traits can be derivable. Serialize is a custom derivable trait.

    Now that we have derived Serialize any instance of our struct can be serialized by serde into the output format of our choice.
***/


#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result};
use serde:: Serialize;

#[derive(Serialize)]
struct IndexResponse {
    message: String,
}

pub struct MessageApp {
    port: u16,
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        // same as writing:
        // MessageApp { 
        //    port: port
        // }
        MessageApp { port }
    }

    pub fn run(&self) -> std::io::Result<()> {
        println!("Starting http server: 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(index)
        })
        .bind(("127.0.0.1", self.port))?
        .workers(8)
        .run()
    }
}

#[get("/")]
fn index(req: HttpRequest) -> Result<web::Json<IndexResponse>> {
    let hello = req
        .headers()
        .get("hello")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_else(|| "world");

    Ok(web::Json(IndexResponse {
        message: hello.to_owned(),
    }))
}