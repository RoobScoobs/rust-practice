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
***/

use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
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

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
        }

        ThreadPool { threads }
    }
}