/***
 * 
 * 
    LISTENING TO THE TCP CONNECTION

    The web server needs to listen to a TCP connection
    and the standard library offers a std::net module

    The code will listen to the address specified at the bind function

    The bind function returns a Result<T, E>, which indicates that binding might fail
    
    (Nonadministrators can listen only on ports higher than 1024,
    so connecting to port 80 without being an admin would cause a failure)

    The incoming method on TcpListener returns an iterator
    that provides a sequence of streams (more specifically, streams of type TcpStream)

    single stream
        - represents an open connection between the client and the server
    
    connection
        - connection is the name for the full request and response process
        in which a client connects to the server, the server generates a response,
        and the server closes the connection
    
    TcpStream will read from itself to see what the client sent
    and then allow to write a response to the stream

    If the stream has any errors calling unwrap will terminate the program

    The reason the stream might receive errors from the incoming method when a client connects to the server
    is that we’re not actually iterating over connections
    
    Instead, connection attempts are being iterated through

***/

use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}