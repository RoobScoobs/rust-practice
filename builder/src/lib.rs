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