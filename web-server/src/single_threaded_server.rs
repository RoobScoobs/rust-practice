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

    READING THE REQUEST

    std::io::prelude enables access to certain traits
    that provides reading from and writing capabilities to the stream

    handle_connection fn

        In the handle_connection function, the stream parameter is mutable
        because the TcpStream instance's internal state might change

        The stream instance keeps track of what data it returns internally,
        and can read more data than asked for and save that data for the next time

        Reading from the stream:

        First Step
            - declare a buffer on the stack to hold the data that is read in
            - 1024 bytes should be sufficient for the sake of this example (buffer management would need to be more complicated irl)
            - passing the buffer to stream.read will read bytes from the TcpStream
              and put them in the buffer

        Second step
            - convert the bytes in the buffer to a string and print the string
            - String::from_utf8_lossy function takes a &[u8] and produces a String from it
        
            
    An invalid UTF-8 sequence will be replaced with � (the U+FFFD replacement character)

    WRITING A RESPONSE

    Responses have the following format:
        HTTP-Version Status-Code Reason-Phrase CRLF
        headers CRLF
        message-body

    So this example will return a simple response
    HTTP/1.1 200 OK\r\n\r\n

    Bind the string literal to the response variable,
    and call as_bytes on the response to convert the string data to bytes

    The write method on stream takes a &[u8]
    and sends those bytes directly down the connection

    Because the write operation could fail, unwrap is used on any error result

    flush() will wait and prevent the program from continuing until all the bytes are written to the connection;
    TcpStream contains an internal buffer to minimize calls to the underlying operating system

    RESPONSE AS HTML

    Using format! can add the file’s contents as the body of the success response
    To ensure a valid HTTP response,
    the Content-Length header - which is set to the size of our response body,
    in this case the size of index.html - is added

    Currently ignoring the request data in buffer
    and just sending back the contents of the HTML file unconditionally
    which means requesting 127.0.0.1:7878/something-else in the browser,
    will return the same HTML response

    VALIDATING THE REQUEST AND SELECTIVELY RESPONDING

    Check that the browser is requesting / before returning the HTML file
    and return an error if the browser requests anything else

    get variable is hardcoded with data corresponding to the / request

    Because raw bytes are being read from the buffer,
    the get variable needs get transformed into a byte string
    and in order to do so can add the b"" byte string syntax at the start of the content data

    If the buffer starts with the bytes in get,
    it means a well-formed request to / is received

    The else block will return a response with the status code 404, along with some HTML
***/

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}{}",
        status_line,
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}