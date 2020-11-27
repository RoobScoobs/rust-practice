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

    IMPLEMENTING THE EXECUTE METHOD

    Change Job from a struct to a type alias for a trait object that holds the type of closure that execute receives

    After creating a new Job instance using the closure passed in the execute method,
    the job is sent down the sending end of the channel

    Calling unwrap on send for the case that sending fails,
    which could happen if all threads stopped from executing,
    meaning the receiving end has stopped receiving new messages
    
    The threads in this case cannot be stopped from executing;
    as long as the pool exists it's fine

    In the worker, the closure being passed to thread::spawn still only references the receiving end of the channel
    Instead, the closure needs to loop forever,
    asking the receiving end of the channel for a job
    and running the job when it gets one

    Inside the closure call lock on the receiver to acquire the mutex,
    and then call unwrap to panic on any errors
    Acquiring a lock might fail if the mutex is in a poisoned state,
    which can happen if some other thread panicked while holding the lock rather than releasing the lock

    If the lock on the mutex is acquired, call recv to receive a Job from the channel

    A final unwrap moves past any errors here as well,
    which might occur if the thread holding the sending side of the channel has shut down,
    similar to how the send method returns Err if the receiving side shuts down

    The call to recv blocks, so if there is no job yet, the current thread will wait until a job becomes available
    
    The Mutex<T> ensures that only one Worker thread at a time is trying to request a job

    WHY A WHILE LET EXPRESSION WON'T RESULT IN THE DESIRED THREADING BEHAVIOR

    while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job; executing.", id);

                job();
            }

    the Mutex struct has no public unlock method
    because the ownership of the lock is based on
    the lifetime of the MutexGuard<T> within the LockResult<MutexGuard<T>> that the lock method returns

    At compile time, the borrow checker can then enforce the rule
    that a resource guarded by a Mutex cannot be accessed unless we hold the lock

    But this implementation can also result in the lock being held longer than intended
    if the lifetime of the MutexGuard<T> isn't carefully considered

    Because the values in the while let expression remain in scope for the duration of the block,
    the lock remains held for the duration of the call to job(),
    meaning other workers cannot receive jobs

    By using loop instead and acquiring the lock without assigning to a variable,
    the temporary MutexGuard returned from the lock method is dropped as soon as the let job statement ends

    This ensures that the lock is held during the call to recv,
    but it is released before the call to job(),
    allowing multiple requests to be serviced concurrently
***/

use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
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

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker {
            id,
            thread
        }
    }
}