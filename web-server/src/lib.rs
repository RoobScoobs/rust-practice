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

    GRACEFUL SHUTDOWN AND CLEANUP

    Implementing the Drop trait will enable calling join on each of the threads in the pool
    so they can finish the requests they’re working on before closing

    In other words, when the pool is dropped,
    the threads should all join to make sure they finish their work

    First, loop through each of the thread pool workers

    Using &mut for this because self is a mutable reference,
    and also need to be able to mutate worker

    For each worker call join() on that worker's thread
    however, an error informs that join cannot be called
    because we only have a mutable borrow of each worker
    and join takes ownership of its argument

    In order to solve this, Worker can hold an
    Option<thread::JoinHandle<()>> instead
    and can call the take method on Option to move the value of the Some variant
    and leave a None variant in its place

    Revise the drop implementation to use if let to destructure the Some
    and get the thread; then call join on the thread

    If a worker’s thread is already None,
    the worker has already had its thread cleaned up, so nothing happens in that case

    SIGNALING THE THREADS TO STOP LISTENING FOR JOBS

    The closures run by the threads of the Worker instances won't shut down the threads
    because they loop forever looking for jobs

    Modify the threads so they listen for either a Job to run
    or a signal that they should stop listening and exit the infinite loop

    Instead of Job instances, the channel will send one of these two enum variants
            NewJob(Job) - holds the Job the thread should run
            Terminate - variant that will cause the thread to exit its loop and stop

    Because there aren't any messages of the Terminate variety being created
    have to change the Drop implementation to send Message::Terminate before calling join on each worker thread 

    Now iterating over the workers twice:
        - once to send one Terminate message for each worker
        - once to call join on each worker’s thread
    
    If a message was sent and join was immediately called in the same loop,
    it couldn't be guaranteed that the worker in the current iteration would be the one to get the message from the channel

    WHY WE NEED SEPARATE LOOPS

    Imagine a scenario with two workers: 
        if there was a single loop to iterate through each worker,
        on the first iteration a terminate message would be sent down the channel
        and join called on the first worker’s thread

        If that first worker was busy processing a request at that moment,
        the second worker would pick up the terminate message from the channel and shut down

        Ultimately left waiting on the first worker to shut down,
        but it never would because the second thread picked up the terminate message

    
***/

use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
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

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread)
        }
    }
}