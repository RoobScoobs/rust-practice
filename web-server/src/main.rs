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

***/

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

struct ThreadPool {
    
}

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

fn handle_connection(stream: TcpStream) {

}