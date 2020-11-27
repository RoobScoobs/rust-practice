/***
 * 
 * 
    TURNING THE SINGLE THREADED SERVER INTO A MULTI-THREADED

    Able to improve throughput with a thread pool as it allows processing connections concurrently

    thread pool
        - a group of spawned threads that are waiting and ready to handle a task
        - when the program receives a new task, it assigns one of the threads in the pool to the task,
          and that thread will process the task
        - the remaining threads in the pool are available to handle any other tasks that come in
          while the first thread is processing
        - when the first thread is done processing its task,
          it’s returned to the pool of idle threads, ready to handle a new task

    THE DESIGN

    Rather than spawning unlimited threads (purposely prevent DoS attacks),
    there'll be a fixed number of threads waiting in the pool

    As requests come in, they’ll be sent to the pool for processing
    The pool will maintain a queue of incoming requests
    
    Each of the threads in the pool will pop off a request from this queue,
    handle the request, and then ask the queue for another request

    This set up can process N requests concurrently, where N is the number of threads

    OTHER OPTIONS TO EXPLORE

    - fork/join model
    - the single-threaded async I/O model

    WHEN DESIGNING CODE

    Write the API of the code so it’s structured in the way you want to call it;
    then implement the functionality within that structure

    Using ThreadPool::new will create a new thread pool with a configurable number of threads

    Within the for loop pool.execute has a similar interface as thread::spawn
    in that it takes a closure the pool should run for each stream

***/

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use web_server::ThreadPool;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
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