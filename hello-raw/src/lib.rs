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
***/

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