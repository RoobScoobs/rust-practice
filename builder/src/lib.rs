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

    FRIENDLY ERROR HANDLING

    The to_compile_errors will handle the transformation of an error
    into something that the procedural macro system can work with

    Assumption is that the errors come as a vector of syn::Errors
    which are the expected types of errors that will be encountered
    i.e. mostly be running into syntax errors

    One nice feature of syn is the associated function syn::Error::to_compile_error
    which converts the error type into a diagnostic error which the compiler will understand
    when returned as a token stream

    The quote! macro uses a syntax similar to the macro_rules macro for generating code,
    except it interpolates variables using the syntax #variable

    This interpolation requires the variable to implement the ToTokens trait

    In this case, the compile_errors are interpolated, however, this variable is an iterator
    therefore, like in delcarative macros, the #(...)* syntax can be used to generate code for each
    element in the compile_errors iterator

    The output of the quote! macro is the interpolated syntax as proc_macro2::TokenStream

    The error function expects a vector of errors
    and in order to make the corresponding Result type easier to write
    can declare a type alias called MultiResult

    SYNTAX ERRORS STRUCT

    A struct called SyntaxErrors is also defined to make working with a vector of errors a little easier
    by implementing helper methods on the struct

    Create an add method which appends a single error to the vector

    This uses generic types to accept anything that can be turned into tokens
    along with anything that can be nicely printed as the description

    The new_spanned function uses the token trees, tts,
    input to capture source information to inform the compiler
    where to draw errors when printing the error out

    A span in the Rust compiler is effectively a region of source code
    Each piece of syntax defines a span so able to bootstrap a span
    if there are some input tokens

    The goal is to inform the compiler as much as possible as to what syntax is causing the problem
    and to describe how best to fix it

    The worst case scenario is when the error just points at #[derive(Builder)]
    and has some opaque message

    The finish method consumes the wrapper struct to return a value of the MultiResult

    The consequence of this is that the ? operator can be used after calling finish
    to report as many errors as they are diagnosed

    GETTING INTO THE PARSER

    The next helper function is called parse_builder_information
    which takes the syn input and returns a result with the builder code or a vector of errors

    The first line brings the Spanned trait into scope,
    so the span method can be called on the input

    The syn::DeriveInput type is destructured into the specific constituent parts that are essential

    Specifically, the data field is matched against the syn::Data enum to see if the item is a struct

    The entire purpose of the function is to ensure that a struct is being derived
    and not an enum or any other possible item

    If a struct is not being derived on, then an error is created and it stops there

    Otherwise, the pieces of data gathered are passed to yet another helper function
    called parse_builder_struct
    
    THE BuilderInfo TYPE

    The name, generics, and vector of fields are all the information needed from the parsed AST
    to be able to generate the desired code

    As an example consider:
        #[derive(Builder)]
        struct Item<T, U>
        where
            T: Default
        {
            a: u32,
            b: T,
            #[builder(required)]
            c: U
        }

    Here name is Item, the generics field captures <T,U> where T: Default,
    and fields contains a: u32, b: T, and #[builder(required)] c: U

    Each of those are wrapped in a suitable data structure that both captures this syntax
    as well as the information about where it lives in the source code

    The fields vector contains a tuple of identifier, type, and attributes
    so for example a: u32 would be something like (Some(a), u32, vec![])

    HANDLING ATTRIBUTES

    The BuilderAttribute is an enum defining all the attributes supported
    
    There's only one supported variant, and it will capture the attribute: #[builder(required)]

    The meaning of this attribute is to specify that a field must be set as part of the build process,
    and therefore a default value should not be used

    For fields not marked required,
    the assumption is that the type of the field implements the Default trait

    The struct BuilderAttributesBody will be a collection of BuilderAttributes,
    which allow for the handling of a list of attributes

    Implementing the Parse trait from syn for the struct will enable bringing in custom logic
    into the work syn already does

    The purpose of the implementation of parse is to remove the parentheses from #[builder(...)]
    so that BuilderAttribute only has to deal with the tokens inside

    It also deals with the logic of a comma separated list here

    The parenthesized!(inside in input) means to take the ParseStream in input,
    remove the parentheses from the outside and the store the inner tokens in inside

    This is a macro defined in syn for this very common scenario
    There are similar parser macros for removing curly braces (braced!) and square brackets (bracketed!)

    The next step is to parse a sequence of BuilderAttribute types separated by commas allowing an optional trailing comma

    The Punctuated<T, P> type is used to handle this very common case of a list of T separated by P
    
    The parse_terminated allows for trailing punctuation
    If there's no need to accept a trailing punctuation, then parse_separated_nonempty can be used

    The final work that needs to be done is to extract the BuilderAttribute types from the punctuated list

    The into_pairs method on Punctuated can accomplish that
    
    It returns an iterator of Pair<BuilderAttribute, Comma>
    and for each of these can call into_value to get the BuilderAttribute out

    Finally, collect is called on the iterator to turn it into the vector that the return type expects

    Now turning to the Parse trait for BuilderAttribute,
    the aim is to check if the attribute is literally required,
    if so then a success is returned, otherwise a failure is declared
    
    Methods from syn are called to turn the input into an Ident
    which is the only thing expected

    If this step fails then an error is returned because of the ? operator

    Then this Ident is compared to "required"
    If there's a match, wrap the input token stream inside the enum variant

    Otherwise using the location of the Ident that is parsed an error is generated saying that there was something unexpected

    FROM syn::Attributes TO THE DESIRED TYPE

    A vector of attributes and vector of errors are defined and only one will be returned

    The Iterator trait has many useful methods, filter_map
    which is used here to both map over an iterator and remove some unwanted items at the same time
    
    The closure passed to filter_map returns an Option<T>

    The resulting iterator will "unwrap" the values that are Some(T) to just be T
    and will not include the None values

    The attributes that are passed as input - ignoring the ones that are not the builder attribute -
    are parsed into the specialized types
    
    The parse2 function is for parsing proc_macro2 types but is otherwise the same as parse for proc_macro types

    Suppose there is: 
        #[something(else), builder(required)]

    The iteration would run through something(else) and builder(required)
    
    The first thing seen is an attr.path of something which is not builder
    so None is returned for that attribute which effectively excludes it from the parsed_attrs result

    The next thing seen has a path that matches builder so the tokens of the attribute,
    which is (required) are parsed into a BuilderAttributeBody, which relies on the Parse trait implementation

    Once that is parsed map(|body| body.0) is called because the Parse trait returns a Result,
    so have to deal with getting inside the Ok variant to pull the Vec<BuilderAttribute> out of the tuple struct wrapper that's put around it

    It's worth mentioning that implementing Parse on Vec<BuilderAttribute> cannot happen as Vec nor Parse is owned by us

    Rust trait implementation rules require that either the trait or the type is owned
    (where ownership means it is defined within the crate with the trail implementation)

    However, this file does own BuilderAttributeBody(Vec<BuilderAttribute>) thus Parse can be implemented on the tuple struct

    FINISHING UP THE PARSING

    The purpose of the parse_builder_struct is to deal with all the various error cases that might occur
    so that in the end if there's a BuilderInfo struct everything is legit for doing code generation

    There's already assurance that we're getting derived on a struct so there's a syn::DataStruct type to work with

    The rest of the input was pulled out of the parsed input
    because it is the only things needed to eventually define a BuilderInfo struct

    First step is to check the attributes defined on the struct itself to see if a builder attribute was used

    There's no support for #[builder(required)] on the entire struct
    so an error is added to the collection of errors if one is seen

    The next step is to get a handle on the fields defined on the struct

    There's no support on the type of struct that has unnamed struct fields
    if for example there's a tuple struct defined such as: 
        struct Foo(String)
    which has one unnamed field

    Therefore, named fields are specifically looked for and its inner data is pulled out
    otherwise an error is added and then all the errors gathered so far are returned

    For each of the named fields, the identifier, type and attributse need to be extracted

    This is done by iterating over the fields
    and then using methods on the field type to get the information wanted

    The attributes_from_syn is used to extract attribute information

    Attributes are looked at for every field,
    so there's the potential to accumulate multiple errors depending on the input

    Finally, errors are returned if encountered,
    or return a successful result containing the BuilderInfo struct 
***/
    
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use std::fmt;
use syn::parenthesized;
use syn::parse::Result as SynResult;

type MultiResult<T> = std::result::Result<T, Vec<syn::Error>>;

enum BuilderAttribute {
    Required(proc_macro2::TokenStream),
}

#[derive(Debug, Default)]
struct SyntaxErrors {
    inner: Vec<syn::Error>,
}

struct BuilderInfo {
    name: syn::Ident,
    generics: syn::Generics,
    fields: Vec<(Option<syn::Ident>, syn::Type, Vec<BuilderAttribute>)>,
}

struct BuilderAttributeBody(Vec<BuilderAttribute>);

impl SyntaxErrors {
    fn add<D, T>(&mut self, tts: T, description: D)
    where
        D: fmt::Display,
        T: quote::ToTokens
    {
        self.inner.push(syn::Error::new_spanned(tts, description));
    }

    fn extend(&mut self, errors: Vec<syn::Error>) {
        self.inner.extend(errors);
    }

    fn finish(self) -> MultiResult<()> {
        if self.inner.is_empty() {
            Ok(())
        } else {
            Err(self.inner)
        }
    } 
}

impl syn::parse::Parse for BuilderAttributeBody {
    fn parse(input: syn::parse::ParseStream) -> SynResult<Self> {
        use syn::punctuated::Punctuated;
        use syn::token::Comma;

        let inside;
        parenthesized!(inside in input);

        let parse_comma_list = Punctuated::<BuilderAttribute, Comma>::parse_terminated;
        let list = parse_comma_list(&inside)?;

        Ok(BuilderAttributeBody(
            list.into_pairs().map(|p| p.into_value()).collect(),
        ))
    }
}

impl syn::parse::Parse for BuilderAttribute {
    fn parse(input: syn::parse::ParseStream) -> SynResult<Self> {
        use syn::Ident;

        let input_tts = input.cursor().token_stream();
        let name: Ident = input.parse()?;

        if name == "required" {
            Ok(BuilderAttribute::Required(input_tts))
        } else {
            Err(syn::Error::new(
                name.span(),
                "expected `required`",
            ))
        }
    }
}

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

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);

    quote! {
        #(#compile_errors)*
    }
}

fn parse_builder_information(ty: syn::DeriveInput) -> MultiResult<BuilderInfo> {
    use syn::spanned::Spanned;
    use syn::Data;

    let span = ty.span();
    let syn::DeriveInput {
        ident,
        generics,
        data,
        attrs,
        ..
    } = ty;

    match data {
        Data::Struct(struct_) => parse_builder_struct(struct_, ident, generics, attrs, span),
        _ => Err(vec![syn::Error::new(
            span,
            "Can only derive `Builder` for a struct",
        )])
    }
}

fn parse_builder_struct(
    struct_: syn::DataStruct,
    name: syn::Ident,
    generics: syn::Generics,
    attrs: Vec<syn::Attribute>,
    span: proc_macro2::Span
) -> MultiResult<BuilderInfo> {
    use syn::Fields;

    let mut errors = SyntaxErrors::default();

    for attr in attributes_from_syn(attrs)? {
        match attr {
            BuilderAttribute::Required(tts) => {
                errors.add(tts, "required is only valid on a field");
            }
        }
    }

    let fields = match struct_.fields {
        Fields::Named(fields) => fields,
        _ => {
            errors.extend(vec![syn::Error::new(
                span,
                "only named fields are supported"
            )]);

            return Err(errors
                .finish()
                .expect_err("just added an error so there should one"));
        }
    };

    let fields = fields
        .named
        .into_iter()
        .map(|f| match attributes_from_syn(f.attrs) {
            Ok(attrs) => (f.ident, f.ty, attrs),
            Err(e) => {
                errors.extend(e);
                (f.ident, f.ty, vec![])
            }
        })
        .collect();

    errors.finish()?;

    Ok(BuilderInfo {
        name,
        generics,
        fields,
    })
}

fn attributes_from_syn(attrs: Vec<syn::Attribute>) -> MultiResult<Vec<BuilderAttribute>> {
    use syn::parse2;

    let mut ours = Vec::new();
    let mut errs = Vec::new();

    let parsed_attrs = attrs.into_iter().filter_map(|attr| {
        if attr.path.is_ident("builder") {
            Some(parse2::<BuilderAttributeBody>(attr.tokens).map(|body| body.0))
        } else {
            None
        }
    });

    for attr in parsed_attrs {
        match attr {
            Ok(v) => ours.extend(v),
            Err(e) => errs.push(e),
        }
    }

    if errs.is_empty() {
        Ok(ours)
    } else {
        Err(errs)
    }
}