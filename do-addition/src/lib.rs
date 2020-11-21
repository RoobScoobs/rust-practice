/***
 * 
    WORKING WITH PRIMATIVES TO UNDERSTAND HOW WASM OPERATES

    Exposing a function which only deals with primative values

    The #[no_mangle] attribute tells the Rust compiler that we want the name of our
    function to be add in the final binary instead of some more complicated name
    that is auto-generated based on the name and types

     Usually don’t have to worry about the exact name of the functions in the compiled executable,
     but because a library is being exposed and it will be callable from JavaScript need to know the actual name to call

     Also place the modifier extern "C" on the function to say that this function 
     uses the right calling conventions that Wasm will understand
     Otherwise this is just a simple publicly exposed Rust function
 ***/

#[no_mangle]
pub extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}