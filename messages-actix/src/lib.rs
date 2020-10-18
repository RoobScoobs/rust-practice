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

    WORKING WITH OPTIONS

    req.headers.get("hello") returns an Option<&HeaderValue>
        - the HeaderValue is a type that abstracts the bytes that actually the data from the request
    
    Option<T> is an enum in the standard library with two variants: Some(T) and None

    After calling .get("hello") we use the function and_then defined on Option to call a function
    If the header exists, we call our closure with the value, otherwise and_then is a no-op on None

    Our closure has to return an Option so that the type in both scenarios matches up.
    
    We call to_str on the &HeaderValue which gives us Result<&str, ToStrError> 
    where ToStrError is a specific error that explains why the conversion to a &str failed.

    We still haven't returned an Option, however Result has a handy function called ok
    which takes data from the Ok variant of a Result
    and puts it inside the Some variant of an Option.
    
    Otherwise it turns the Err variant of the Result into the None variant of Option and discards the error

    Finally, we want our hello variable to just contain a &str, but we have an Option<&str>.

    We can use a helper defined on Option called unwrap_or_else
    which will unwrap and return data inside the Some variant of the Option if it is set
    otherwise in the case of the None variant, this function will call the provided function and return the result.

    RETURNING THE VALUE OF THE HELLO HEADER

    The type of message is String (from our IndexResponse struct) and the type of the hello variable is &str (borrowed string slice).
    So we need to convert our borrowed string into an owned string so that we can return the data as a response.

    Find that all of to_owned(), to_string(), and into() would work to do this conversion

    ACTIX WORKERS

    Actix by default will create a number of workers to enable handling concurrent requests. 
    One piece of state we are going to maintain is a unique usize for each worker.

    Each worker thread gets its own instance of the AppState state struct which we created.
    Actix takes an application factory because it will create many instances of the application, and
    therefore many instances of the state struct.

    STATIC VS CONST
    
    Both live for the entire lifetime of the program

    Items marked with const are effectively inlined at each site they are used.
    Therefore references to the same constant do not necessarily point to the same memory address.

    On the other hand, static items are not inlined,
    they have a fixed address as there is only one instance for each value.
    Hence static must be used for a shared global variable.

    It is possible to have static mut variables,
    but mutable global variables are bad and
    therefore in order to read/write mutable statics requires the use of the unsafe keyword.

    Both static and const variables must have their types given explicitly,
    so we write the type AtomicUsize for our SERVER_COUNTER variable

    DEFINING THE STATE

    The first part of the state will be set from the atomic usize.

    The second piece of data will keep track of the number of requests seen by the particular worker that owns this instance of state.

    The last piece of state is going to be a vector of strings that represent messages shared across all of the workers.

    We want each worker thread to be able to read and write this state,
    and we want updates to be shared amongst the workers.
    
    INTERIOR MUTATBILITY

    Rust memory safety is based on this rule: Given an object T, it is only possible to have one of the following:
        - having several immutable references (&T) to the object (also known as aliasing)
        - having one mutable reference (&mut T) to the object (also known as mutability)

    Sometimes it is required to have multiple references to an object and mutate it.

    Rust has a pattern for mutating a piece of data inside a struct
    which itself is immutable known as interior mutability.

    Two special types enable this, Cell and RefCell

    Cell --- implements interior mutability by moving values in and out of a shared memory location.

    RefCell --- implements interior mutability by using borrow checking at runtime
    to enforce the constraint that only one mutable reference can be live at any given time.

    SHARING ACROSS THREADS

    We can ensure mutually exclusive access to the vector of strings by creating a Mutex that wraps our vector.

    Mutex<Vec<String>> is a type that provides an interface for
    coordinating access to the inner object (Vec<String>) across multiple threads.

    Typically each value in Rust has a single owner,
    but for this situation we want each thread to be an owner of the data
    so that the vector lives until the last worker thread exits.
    The mechanism for this in Rust is to use a reference counted pointer.

    There are two variants: Rc and Arc

    You cannot share an Rc across threads, but you can share an Arc.
    Otherwise they are equivalent.

    EXTRACTING DATA FROM REQUESTS

    Extractors are types that implement the FromRequest trait which allow types to define how they are constructed from a request.

    Any type that implements FromRequest can technically fail to extract said type and
    thus uses Result in the implementation.

    web::Data<AppState> is just a wrapper around our state that handles the FromRequest implementation.
    This wrapper implements the Deref trait so that we
    can treat a value of Data<AppState> effectively as if it was a value of AppState.

    The reason that we cannot mutate request_count directly is that our state variable (in the index function) is an immutable reference
    
    EFFECTIVELY WORKING WITH LOCKS

    Arc implements Deref so that when we call a method on state.messages
    it will automatically get called on the value of type Mutex<Vec<String>>.
    To get access to the data inside the mutex we call the lock method on the mutex.

    The lock method blocks until the underlying operating system mutex is not held by another thread.
    This method --- lock() --- returns a Result wrapped around a MutexGuard which is wrapped around our data.

    We choose to use the unwrap method on Result which pulls data out of the Ok variant, but instead panics on the Err variant.
    Often you will see lock().unwrap() used with mutexes

    The type of the variable we get from state.messages.lock().unwrap() is actually a MutexGuard<Vec<String>>

    HOW MUTEXES WORK IN RUST

    RAII (Resource Acquisitions Is Initialization) is a pattern for managing resources which is central to Rust.
    In particular, when a value goes out of scope, a special method called drop is called by the compiler
    if the type of the value implements the Drop trait

    For a MutexGuard, the mutex is locked when the guard is constructed
    and the lock is unlocked in the guard’s drop method.

    The data is only accessible while the lock is locked.

    RESPONDING WITH DATA

    Finally, we construct an instance of our response struct to be serialized and returned as JSON.

    The clone method (called on the variable ms) creates an explicit copy of a value if the type implements the Clone trait.
    We cannot just pass the messages vector directly because that would move ownership and that is not what we want to do.

    We create the shared messages vector outside of the application factory closure with the line: 
    let messages = Arc::new(Mutex::new(vec![]));

    We do this so that each worker can actually share the same messages array 
    rather than each of them creating their own vector which would be unconnected from the other workers.

    To add state to the application we use the data method on App and pass in the initial value of our state.
    This is what makes the web::Data<AppState> extractor work.
    
    CONSTRUCTING OUR STATE
    
    The strongest ordering is SeqCst which stands for sequentially consistent and
    it's one of the options of how atomic operations synchronize memory across threads.

    The best advice is to use SeqCst until you profile your code, 
    find out that this is a hot spot, and
    then can prove that you are able to use one of the weaker orderings based on your access pattern.

    Each thread will own its own Cell
    so we just construct the cell inside the application factory closure
    which is executed by the worker thread and
    therefore has affinity to that thread which is what we desire.

    Finally, we clone the Arc value that wraps the shared messages value
    which means that we create a new pointer to the shared data.

    RECEIVING INPUT

    Import Deserialize item from serde so that we can derive the ability to construct structs
    from JSON data

    Our input will just be of the form {"message": "some data"}.
    We will then be able to use Serde to turn JSON data with that format into instances of our struct.

    Note that we declare our message vector variable ms to be mutable so that we can call push
    which is safe because as we are holding the mutex we have exclusive read/write access to the vector.

    We have to clone the message as we push it into the vector
    because this vector owns each element and
    we only have a borrowed reference to our PostInput data.

    DEFINING THE ROUTE TO OUR POST HANDLER

    The service method takes a resource definition.

    First we create a resource for the specific path /send with web::resource("/send").

    The data method is used for specifying route specific data or for configuring route specific extractors.
        - here we're setting a limit on the number of bytes to deserialize to 4096 bytes

    We then declare the route configuration for this resource by passing route data to the route method
        - use web::post() to say that this route requires a POST request

    Finally, to is called with our handler function post to indicate which function to call for this route.

    The clear handler is similar to our index request but instead of pushing a new message onto our vector 
    we mutate it by calling clear() to remove all messages

    CUSTOM ERROR HANDLING

    The type for the error handler is defined by the JsonConfig type
    and is not as flexible as the other handlers that we can define.

    We cannot use extractors to get different input, and
    we have to return the Error type from actix_web.


    GENERIC RETURN TYPES

    Actix uses a type safe bag of additional data attached to requests called extensions.

    The state is just the value inside of the extensions with type web::Data<AppState>

    The extensions have a generic function called get which has the signature:
        fn get<T>(&self) -> Option<&T

    This function returns a reference to a type that was previously stored as an extension.
    It is up to the caller (i.e. us) to say what type we want to get back.

    Have to give the compiler some help by putting a type annotation somewhere so that get knows what type we want.

    By using the ::<> turbofish syntax, namely
        get::<web::Data<AppState>>
    we can annotate the type on the get method, and
    so this means call get<T>() with T bound to the type web::Data<AppState>

    We call unwrap on the Option we get back to get direct access to our state.

    CREATING USERFUL ERRORS

    The format macro takes a format string along with the necessary variables to fill in the placeholders
    and returns an owned String.

    In a real app, you would probably want a more user friendly message than even just displaying this error.
    One approach would be to match on the different variants of the JsonPayloadError enum
    and make our own message in the different scenarios.

    InternalError is a helper provided by actix to wrap any error and turn it into a custom response.

    So we call the constructor from_response, passing in the JsonPayloadError
    which gets stored as the cause of the error, and
    then the second argument is the custom response we want to return.

    The HttpResponse struct has a variety of helpers for building responses,
    one of which is BadRequest which sets the status code to 400.

    BadRequest has a method that returns a response builder
    which has a json method that can take anything that is serializable into JSON
    and sets it as the response body.

***/


#[macro_use]
extern crate actix_web;

use actix_web::{
    error::{Error, InternalError, JsonPayloadError},
    middleware, web, App, HttpResponse, HttpRequest, HttpServer, Result,
};
use serde:: {Deserialize, Serialize};
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}

#[derive(Serialize)]
struct IndexResponse {
    server_id: usize,
    request_count: usize,
    messages: Vec<String>,
}

pub struct MessageApp {
    port: u16,
}

#[derive(Deserialize)]
struct PostInput {
    message: String,
}

#[derive(Serialize)]
struct PostResponse {
    server_id: usize,
    request_count: usize,
    message: String,
}

#[derive(Serialize)]
struct PostError {
    server_id: usize,
    request_count: usize,
    error: String,
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
        let messages = Arc::new(Mutex::new(vec![]));
        println!("Starting http server: 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                .data(AppState {
                    server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
                    request_count: Cell::new(0),
                    messages: messages.clone(),
                })
                .wrap(middleware::Logger::default())
                .service(index)
                .service(
                    web::resource("/send")
                        .data(web::JsonConfig::default().limit(4096))
                        .route(web::post().to(post))
                )
                .service(clear)
        })
        .bind(("127.0.0.1", self.port))?
        .workers(8)
        .run()
    }
}

#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let ms = state.messages.lock().unwrap();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        messages: ms.clone(),
    }))
}

#[post("/clear")]
fn clear(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let mut ms = state.messages.lock().unwrap();
    ms.clear();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        messages: vec![],
    }))
}

fn post(msg: web::Json<PostInput>, state: web::Data<AppState>) -> Result<web::Json<PostResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let mut ms = state.messages.lock().unwrap();
    ms.push(msg.message.clone());

    Ok(web::Json(PostResponse {
        server_id: state.server_id,
        request_count,
        message: msg.message.clone(),
    }))
}

fn post_error(err: JsonPayloadError, req: &HttpRequest) -> Error {
    let extns = req.extensions();
    let state = extns.get::<web::Data<AppState>>().unwrap();
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let post_error = PostError {
        server_id: state.server_id,
        request_count,
        error: format!("{}", err),
    };
    
    InternalError::from_response(err, HttpResponse::BadRequest().json(post_error)).into()
}

// #[get("/")]
// fn index(req: HttpRequest) -> Result<web::Json<IndexResponse>> {
//     let hello = req
//         .headers()
//         .get("hello")
//         .and_then(|v| v.to_str().ok())
//         .unwrap_or_else(|| "world");

//     Ok(web::Json(IndexResponse {
//         message: hello.to_owned(),
//     }))
// }