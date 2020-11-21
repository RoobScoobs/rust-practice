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