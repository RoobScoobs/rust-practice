/***
    ARRAYS:

    Arrays in Rust are a homogeneous container (all elements have the same type) with a fixed size. 
    This allows it to be stack allocated.

    TYPE ANNOTATION:
    
    Type annotations are written with a colon after the variable name followed by the type. 
    We see that the size of the array (5) is part of the type.

    Type inference does not operate on function signatures so you must fully specify the types of all inputs and the output.

    std::vec: 
    A vector is similar to an array in that it stores a single type of element in a contiguous memory block.
    However, the memory used by a vector is heap allocated and can therefore grow and shrink at runtime.
    
    The type of a vector is Vec<T> where T is a generic type that represents the types of the elements.
    Therefore, Vec<i32> and Vec<u8> are different types, but a Vec<u8> with four elements is the same type as one with five elements.

    std::slice:
    Slices are a dynamically sized view into a sequence. 
    Therefore, you can have a slice which references an array or a vector and treat them the same.

    mut
    Mutability is a property of the variable or reference not of the object itself.

    RANGES
    Ranges can be constructed with using the syntax start..end or start..=end. 
    Both start and end are optional
    By default the range is inclusive on the left (i.e. includes start), and exclusive 
    on the right (i.e. does not include end).
    The = after the two dots makes it so the range includes the end point

    collect()
    a generic function over the return type, so the caller gets to determine what they want

    ATTRIBUTES
    They come in two forms #[...] and #![...] which annotate the item they precede.
***/

// pub keyword is a privacy identifier which specifies that 
// this function should be publicly accessible to user’s of our crate
pub fn print(limit: u8) {
    let numbers = generate_sequence(limit);
    output_sequence(&numbers);
}

/***

    Rust has a few different modes of passing arguments to functions. 
    
    The biggest distinction being that Rust differentiates between:
        - a function temporarily having access to a variable (borrowing) and
        - having ownership of a variable

    The default behavior is for a function to take input by value and hence ownership of the variable is moved into the function.
    
    The exception to this rule being if the type implements a special trait called Copy,
    in which case the input is copied into the function and
    therefore the caller still maintains ownership of the variable.

    The Rust compilation model does not allow functions to directly take arguments of an unknown size.
    In order to access this slice of unknown size with something of a known size we use indirection and pass a reference to the slice
    rather than the slice itself.
    A reference to a slice of u8 values has type &[u8] which has a known size at compile time.

***/

fn generate_sequence(limit: u8) -> Vec<u8> {
    // this function can be used to turn any iterator into basically any collection
    // by calling collect we are turning a range into a vector
    // the type inference sees that the return type needs to be a Vec<u8> and
    // therefore ensures that collect generates that collection
    (1..=limit).collect()
}

fn generate_sequence_2(limit: u8) -> Vec<u8> {
    
    // By default a vector created with new, is the same as one created with vec![], and does not allocate.
    // Therefore, unless you actually put something into a vector it does not use any memory

    // we declare numbers to be a mutable variable that holds an empty vector.
    let mut numbers = Vec::new();

    // iterator here is a range object
    // We want the numbers starting at 1 up to limit, including the limit
    for n in 1..=limit {
        // mut allows us to call numbers.push(n) because push is a method that requires the receiver to be mutable
        numbers.push(n);
    }

    // The expression that evaluates to the vector numbers is written without a semicolon and means to return that value
    // here a vector unsigned 8-bit type is returned as stated above in the function signature
    numbers
}

// slice example
fn output_sequence(numbers: &[u8]) {
    for n in numbers {
        println!("{}", n);
    }
}

// array example
fn output_sequence_2(numbers: [u8; 5]) {
    for n in numbers.iter() {
        println!("{}", n);
    }
}

// vector example
fn output_sequence_3(numbers: Vec<u8>) {
    for n in numbers.iter() {
        println!("{}", n);
    }
}

pub fn print_2() {

    // type of [{integer}; 5]

    let numbers = vec! [
        1,
        2,
        3,
        4,
        5
    ];

    // let () = numbers;

    // explicitly calling the inter method to enable use of a for loop  
    // for n in numbers.inter()

    // Here however, Vec implements a trait that tells the compiler how to convert it 
    // into an iterator in places where that is necessary like in a for loop
    for n in numbers {
        println!("{}", n);
    }
}

pub fn print_3() {
    let numbers = [1, 2, 3, 4, 5];
    output_sequence_2(numbers);
}

pub fn print_4() {
    // suppose we wanted to use a Vector type for our array
    let numbers = vec![1, 2, 3, 4, 5];
    output_sequence_3(numbers);
}

pub fn print_5() {
    let vector_numbers = vec![1, 2, 3, 4, 5];
    let array_numbers = [1, 2, 3, 4, 5];

    // by adding "&" we create a slice that represents read-only access
    // to the entire sequence for both the vector and array

    // we are no longer transferring ownership into the function output_sequence 
    // instead we are lending read-only access to that function
    output_sequence(&vector_numbers);
    output_sequence(&array_numbers);
}

#[test]
// use cargo test in CLI
fn generated_sequence_should_work() {
    let result = generate_sequence(7);

    // use assert_eq to ensure that the output of our generate_sequence function is what we expect it to be
    assert_eq!(result, &[1, 2, 3, 4, 5, 6, 7]);
}