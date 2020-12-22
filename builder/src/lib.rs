/***
 * 
 * 
 * 
    INITIAL SET-UP

    The proc-macro = true in the manifest tells the compiler to inject a specific dependency into the crate
    and also the procedular macros that are exported can be used by other crates

    The downside to code using proc-macro is that it can only execute in procedular macro contexts

    In other words, tools for understanding Rust syntax in normal code cannot be used
    unless it doesn't depend on proc-macro

    Furthermore, cannot unit test the proc-macro code

    A separate crate, proc-macro2, was created to fix these problems

    The foundational crates for parsing Rust syntax, syn,
    and generating Rust syntax, quote, are written against the proc-macro2 crate

    SCAFFOLDING

    The 2018 edition did away with the need for extern crate definitions

    However because proc_macro is a builtin crate, still have to use the extern crate syntax
    Don't have to use extern crate with std but proc_macro didn't get the special treatment

    The public interface to the library: builder_derive
    is the entirety of what is exported by the library

    syn::parse is called to turn the input into a data structure representing 
    the AST (Abstract Syntax Tree) of the item defined on

    Then that is passed on to the implementation piece, impl_builder_macro

    The type signature of impl_builder_macro along with type inference is what tells
    syn::parse what type to turn the input into

    Docs on the syn crate: https://docs.rs/syn/1.0.55/syn/index.html

    Using expect to panic if the input cannot be parsed
    It is common to panic or use the compile_error macro in procedular macros

    This is because the code is executing at compile time
    so causing a panic is sometimes the only thing to do to stop the compilation process
    when unable to proceed

    THE proc_macro_derive ATTRIBUTE

    This attribute takes the name of the derive as the first argument
    which is Builder in this case

    This makes so it's possible to write:
        #[derive(Builder)]
        struct Request {
            ...
        }

    The second argument to proc_macro_derive is optional
    and defines helper attributes

    The syntax here is to write attributes(...) with a comma separated list of attributes
    that are defined inside the parentheses

    Here a single attribute called builder is defined so it's possible to write:
        #[derive(Builder)]
        struct Request {
            #[builder]
            pub x: i32,
            ...
        }

    Attributes can also take arguments:
        #[derive(Builder)]
        struct Request {
            #[builder(foo, bar=32)]
            pub x: i32,
            ...
        }

    TYPE SIGNATURE

    The function builder_derive has the signature (TokenStream) -> TokenStream
    which is the form that every custom derive must implement

    The input is the item that the derive is defined on
    and the return value is appended to the module or block
    where that item is defined

    Custom attribute macros have a signature of (TokenStream, TokenStream) -> TokenStream
    where the first argument is the arguments to the attribute itself
    and the second attribute is the item the attribute is on

    The return value replaces the item with an arbitrary number of items

    For example,
        #[get("/")]
        fn foobar() {}

    means that there is a procedural macro that defines a function get
    which will receive the token stream representing ("/") as its first argument
    and the token stream representing fn foobar() {} as its second argument

    The token stream output from that function will replace all of that code

    Function-like procedular macros have the same signature as a derive macro
    (TokenStream) -> TokenStream, where the input is the entirety of the macro invocation
    but instead of getting appended to the module, the token stream that is returned
    replaces the input at the same location in the source

    THE impl_builder_macro FUNCTION

    The purpose of this function is to move into the world of proc_macro2
    by passing the syn input into a function which only operates in this other world

    The into implementation of a type will help convert back into the proc_macro world
    which is then used to retrun the expected TokenStream

    The proc_macro2::TokenStream type impelements the Into trait to get a
    proc_macro::TokenStream so the expectation is to get proc_macro2::TokenStream values
    that simply need to call into on


***/

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use std::fmt;
use syn::parenthesized;
use syn::parse::Result as SynResult;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Could not parse type to derive Builder for");

    impl_builder_macro(ast)
}

fn impl_builder_macro(ty: syn::DeriveInput) -> TokenStream {
    match parse_builder_information(ty) {
        Ok(info) => info.into(),
        Err(e) => to_compile_errors(e).into(),
    }
}