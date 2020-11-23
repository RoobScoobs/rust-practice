/***
 * 
 * 
    GOAL

    To expose the function greet which takes a &str as input and returns a new heap allocated String
    
    Did not specify #[no_mangle] because we are not going to call this directly
    and therefore we want Rust to mangle the name

    Webassembly does not understand &str or String types,
    it can just take 0 or more numeric inputs and return a single numeric output

    So have to figure out how to turn &str into 1 or more numeric inputs
    and String into a single numeric output

    WHAT IS THE GREET WRAPPER DOING

    Turned the string input into a pointer (which is just an integer) to the beginning of where the string lives in memory
    and the length of the string (also an integer)

    Within the unsafe block perform some work to turn the input into a &str (string slice) which is then passed
    to the original greet function

    In order to change the String output of the greet function to an integer can use the Box type
    which will transform it to a pointer

    So the greet wrapper function only uses integer inputs and output
    but effectively exposes the higher level greet function we wrote that operates on strings

    INPUTS & OUTPUTS OF THE WRAPPER FUNCTION

    Pointers are not common in most of Rust because references are usually used when sharing data

    However, when working across FFI (Foreign Function Interface) boundaries, e.g. when talking to JavaScript,
    pointers are used directly as they are a concept shared on both sides

    We take a pointer to a memory region containing bytes (a u8 value is just one byte)
    which represents a string and the length of that string

    Because dealing with raw pointers is unsafe in Rust,
    it is the responsibility of the caller to ensure that the pointer is valid
    and that the length is correct

    IMPLICIT CONTRACT

    The return type is a pointer to a mutable String, i.e. a heap allocated String
    (String itself is already on the heap so this is an extra allocation)

    It is an implicit contract of our function that the caller is responsible for making sure
    that this pointer gets passed back to Rust at some point to clean up the memory on the heap

    POINTER CONVERSIONS

    Standard library provides a function std::slice::from_raw_parts which will give a &[u8]

    Given that we have a &[u8] we want to turn this into a &str
    which is also valid UTF-8
    
    There are two mechanism for doing this:
        std::str::from_utf8
            - this one performs validity checks on the input to ensure that the passsed in slice really is UTF-8
            - returns Result<&str, Utf8Error>
        std::str::from_utf8_unchecked
            - doesn't have the same validity checks; trusts that it is valid UTF-8
            - simply returns the &str

    IMPLICIT CONTRACT CONT'D

    The pointer passed in must be to a sequence of bytes, the length must match the length of that sequence,
    and the sequence of bytes must be valid UTF-8
    If any of those are violated then undefined behavior can result

    USING UNSAFE CODE IN RUST

    Keep the block as small as possible to get back into safe Rust quickly
    This makes auditing the code and understanding the necessary invariants
    much more manageable

    HOW TO GIVE A POINTER TO THE STRING RETURNED FROM GREET

    Calling greet() returns the relevant string back, but need to somehow give a pointer to this String
    and ensure that the underlying memory is no deallocated

    Use Box::new to create a new heap allocation to hold the String

    Note that the String is already on the heap by its nature, but we create a new heap allocation to own the String
    because we can keep the String alive if we can keep the Box alive as it becomes the owner of the String
    
    We do this by using the associated function Box::into_raw
    This function consumes the Box and returns exactly the raw pointer we want

    Whoever gets this raw pointer is responsible for managing the memory
    and must do something to ensure the box gets destroyed when it is no longer needed,
    otherwise memory will be leaked

    MEMORY ALLOCATION AND DEALLOCATION FUNCTIONS

    __malloc fn

        This function should be publicly accessible, using the “C” calling convention, 
        and the name should be __malloc so we specify #[no_mangle]

        Take a size as input (we use the byte length of our string)
        and return a pointer to some allocated bytes of that size

        The first thing to do is get the minimum alignment for a usize based on the Application Binary Interface (ABI)

        Need this to pass to the Layout constructor because in order to allocate memory you need both a size an an alignment

        Next thing to do is generate a memory layout for the particular size and alignment

        This can fail and return an error if the alignment is bad (zero or not a power of two) or if size is too big, otherwise this should succeed

        Given the layout, can then proceed to actually allocating memory

        If the resulting size of the layout is not positive
        then there's no need to allocate anything,
        and in this case the alignment is cast to the correct return type (align as *mut u8)

        In fact calling alloc with a zero sized Layout could lead to undefined behavior depending on the architecture

        Otherwise there's a real region of memory that needs to be allocated
        so the alloc function provided by the standard library can be used

        It is possible to customize the allocator used, but by default a standard one is used per platform

        Get back a pointer from alloc --- alloc(layout)
        which is the location of the newly allocated region of memory of the size and alignment specified by our layout

        Only return this pointer if it is not null
        A null pointer returned from alloc most likely means you are out of memory

        **Note**
            If you use any method that can panic in your Rust code,
            even if you definitely never panic, your Wasm module will increase quite a bit in size
            because of extra code related to panics

            There are non-panicing alternatives to a lot of methods
            and there are other things you can do in these scenarios
            It is possible to configure your code so that you are not allowed to panic,
            notably by using no_std which means disallowing anything from the std module,
            but that can be extreme (although necessary in some environments)

        
***/

use std::alloc::{alloc, dealloc, Layout};
use std::mem;

pub extern "C" fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[export_name = "greet"]
pub extern "C" fn __greet_wrapper(
    arg0_ptr: *const u8,
    arg0_len: usize
) -> *mut String {
    let arg0 = unsafe {
        let slice = std::slice::from_raw_parts(arg0_ptr, arg0_len);
        std::str::from_utf8_unchecked(slice)
    };
    let _ret = greet(arg0);
    Box::into_raw(Box::new(_ret))
}

#[no_mangle]
pub extern "C" fn __malloc(size: usize) -> *mut u8 {
    let align = mem::align_of::<usize>();

    if let Ok(layout) = Layout::from_size_align(size, align) {
        unsafe {
            if layout.size() > 0 {
                let ptr = alloc(layout);
                if !ptr.is_null() {
                    return ptr
                }
            }
            else {
                return align as *mut u8
            }
        }
    }

    panic!("malloc failed")
}