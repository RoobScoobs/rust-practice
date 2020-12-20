/***
 * 
 * 
 *
    MACROS

    This concept comes from Lisp, and in essence it's a metaprogramming technique
    in which syntax can be operated on as data

    In other words, a macro is a thing which takes code as input
    and produces code as output

    RUST MACRO FEATURES

    - declarative macros
    - procedural custom derive macros
    - procedural attribute macros
    - procedural function-like macros

    PRIMARY REASONS TO USE MACROS

    If you want a function with a variable number of arguments, commonly called variadic functions

    Secondly, if you want to do something that must happen at compile time, like implement a trait

    It's impossible to write a normal function that does either of these things in regular Rust

    DECLARATIVE MACROS

    This type of macro is a declarative style language inspired by Scheme
    
    Can think of it as being similar to a big match statement
    where the conditions are matching on syntactic constructs
    and the result is the code to generate

    A MACRO THAT CREATES A VECTOR WITH ANY ARBITRARY NUMBER OF ELEMENTS
    
    The syntax for creating a macro is to invoke a macro called macro_rules
    with the name of the new macro, myvec in this case, followed by braces to denote the body of the macro

    The body of the macro is just a series of rules with the left side of an arrow, =>,
    indicating what pattern to match, and the right side specifying what code to generate

    The patterns are checked in order from top to bottom,
    so it moves from specific to more general
    so that the specific patterns get a chance to match
    
    THE FIRST MATCH ARM ($($x:expr),*)

    The outer set of parentheses exists to denote the entire pattern

    The inside $x:expr means to match a Rust expression
    and bind it to the variable $x

    The outer part $(...),* means to match zero or more comma separated things inside the parentheses

    So the complete pattern means to match zero or more comma separated expressions,
    with each expression bound to the variable $x

    The right hand side is surrounded by parentheses as well
    which signifies the enclosure of the entirety of the code to generate

    Also have curly braces surrounding the code,
    which means to literally generate a set of curly braces around the code
    so that the output is a block

    The outer parentheses can actually be an balanced brakcet, i.e. (), [], or {}
    are all acceptable in that position

    It's possible to see ($($x:expr),*) => {{ ... }},
    which still means to generate a single block
    The outer braces are there
    so that the compiler knows where the right hand side begins and ends

    The rest of the right hand shows the creation of the vector, what looks like the push method,
    and then the return of that vector

    $(v.push($x);)* - this syntax means to repeat the code inside the parentheses
                      for each repetition captured in the match arm

    Within those parentheses the expression captured will be substituted directly for $x

    In the end let a = myvec![1, 2, 3];

    will expand to:

    let a = {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v
    };

    THE SECOND MATCH ARM $($x:expr,)*

    What if the macro was written as let a = myvec![1, 2, 3,];
    
    The trailing comma is where the second match arm is triggered
    because the comma is inside the repetition, $(...)*

    In this case, there's an expectation to see expressions followed by commans
    and this arm will only match if there's an explicit trailing comma

    The right hand side will convert the comma trailing version to the version without a trailing comma
    and then rely on the previous pattern to match

    This is accomplished by recursively calling the macro
    and expanding $($x:expr,)* to $($x),*

    Moving the comma from inside the repetition to outside means
    to take it from a list of expressions each followed by a comma to
    a list of expressions separated by commas

    EXPANDING A MACRO

    In order to see what the macro expands into
    to see if the generated code matches expectations
    can use: cargo expand

    Running the command from the root of the crate should show all macros expanded out

    MACROS BY EXAMPLE

    https://doc.rust-lang.org/1.30.0/reference/macros-by-example.html

    PROCEDURAL MACROS

    As of Rust 1.30 this type of macro became available on the stable release

    There are 3 types of procedular macros
        - custom derive
        - attribute-like
        - function-like

    CUSTOM DERIVE

    It means that a trait like MyCoolTrait can be created and used in a way that maeks the following code work:
        #[derive(MyCoolTrait)]
        struct SomeStruct;

    SomeStruct will implement MyCoolTrait automatically by having the implementation generated at compile time

    This works by sending the syntax of the item the derive is placed on to some code
    that returns new syntax which will be added to the source alongside the item

    ATTRIBUTE-LIKE

    Attributes are the annotations inside the syntax #[...]
    
    For example #[derive(Debug)] is an attribute
    It is the derive attribute which takes arguments
    
    Unlike the custom derive macros which define the arguments to the derive attribute operator,
    can instead create new attributes
    
    Example from the blog_actix web server was defining route information
        #[get("/lookup/{index}")]
        fn lookup(...){}

    The get attribute is custom and is implemented via a procedural macro

    This type of macro is a function that takes the arguments to the attribute
    as raw syntax as well as the item it is being defined on as syntax
    and then generates code

    FUNCTION-LIKE

    An example of the type may look like:
        gen_object! {
            class Foo: SomeThing {
                x: u32,
                y: RefCell<i16>,
            }

            impl Foo {
                ...
            }
        }

    This means that gen_object takes all of the subsequent syntax as input
    and generates new code to replace it
***/

macro_rules! myvec {
    ($($x:expr),*) => ({
        let mut v = Vec::new();
        $(v.push($x);)*
        v
    });
    ($($x:expr,)*) => (myvec![$($x),*])
}

fn main() {
    let a = myvec![1, 2, 3, 4,];
}
