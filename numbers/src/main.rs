fn main() {
    // we still must refer to the functions in the crate by the name of the crate,
    // numbers in this case

    // colon operator :: is used for separating items in the hierarchy of modules

    /***
        To resolve an item, be it a type or function, you start with the name of the crate,
        followed by the module path to get to the item, and finally the name of the item.

        Each part of this path is separated by ::

        For example, to get a handle to the current thread you can call the function std::thread::current
    ***/

    numbers::print(7);
}
