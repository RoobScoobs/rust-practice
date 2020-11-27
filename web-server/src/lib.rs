/***
 * 
 * 
    BUILDING THE THREADPOOL STRUCT

    ThreadPool implementation will be independent of the kind of work the web server is doing;
    thus it can be in its own library crate

    VALIDATING THE NUMBER OF THREADS IN NEW

    Chose an unsigned type for the size parameter,
    because a pool with a negative number of threads makes no sense

    However, a pool with zero threads also makes no sense, so add validation for zero

    Added some documentation for the ThreadPool with doc comments (///)

    Note that good documentation practices include adding a section
    that calls out the situations in which the function can panic

    scaffold and view documentation by running cargo doc --open (incredibly dope feature!)

    HOW TO STORE A THREAD?

    Looking at the thread::spawn signature the function returns a JoinHandle<T>,
    where T is the type that the closure returns

        pub fn spawn<F, T>(f: F) -> JoinHandle<T>
            where
                F: FnOnce() -> T + Send + 'static,
                T: Send + 'static
    
    In our case, the closures we’re passing to the thread pool will handle the connection
    and not return anything, so T will be the unit type ()

    The field threads in our ThreadPool struct will hold a vector of thread::JoinHandle<()> instances

    Given that the type in the vector is thread::JoinHandle, must bring into scope std::thread
    
    Once a valid size is received, use the Vec::with_capacity fn
    to essentially perform the same task as Vec::new, however, with_capacity
    preallocates space in the vector, which is more efficient

    A WORKER STRUCT RESPONSIBLE FOR SENDING CODE FROM THE THREADPOOL TO A THREAD

    The standard library provides thread::spawn as a way to create threads,
    and thread::spawn expects to get some code the thread should run as soon as the thread is created

    However, in this example, want the ability to create the threads
    and have them wait for code that will be sent later

    To implement this behavior a data structure called Worker is introduced,
    which is a common term in pooling implementations 

    Instead of storing a vector of JoinHandle<()> instances in the thread pool,
    instances of the Worker struct will be stored

    Each Worker will store a single JoinHandle<()> instance

    A Worker will have a method that will take a closure of code to run
    and send it to the already running thread for execution

    SENDING REQUESTS TO THREADS VIA CHANNELS

    Channels - a simple way to communicate between two threads

    Plan:
        - the ThreadPool will create a channel and hold on to the sending side of the channel
        - each Worker will hold on to the receiving side of the channel
        - create a new Job struct that will hold the closures we want to send down the channel
        - the execute method will send the job it wants to execute down the sending side of the channel
        - in its thread, the Worker will loop over its receiving side of the channel and execute the closures of any jobs it receives

    Channel implementation that Rust provides is multiple producer, single consumer
    thus cannot pass receiver to multiple Worker instances; in other words we can't
    clone the consuming end of the channel

    The aim is to distribute the jobs across threads by sharing the single receiver among all the workers

    Additionally, taking a job off the channel queue involves mutating the receiver,
    so the threads need a safe way to share and modify receiver

    To share ownership across multiple threads and allow the threads to mutate the value,
    the thread-safe smart pointer Arc<Mutex<T>> can be used

    The Arc type will let multiple workers own the receiver,
    and Mutex will ensure that only one worker gets a job from the receiver at a time
***/

use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

struct Job {

}

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero
     
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker {
            id,
            thread
        }
    }
}