/***
 * 
 * 
 * 
    SYNTAX MODULE

    It's singular purpose is to load syntax and theme sets
    This module will expose a build function that takes care of the set of syntaxes and themes

    Syntect uses a builder pattern for constructing a syntax set
    
    First create the HTTP syntax and then the JSON syntax

    The provided load_defaults method is used for the theme to get a decent set of themes to return

    The code to construct each syntax definition is the same except for the path to the file
    which is brought in via include_str!

    The path used here is relative to the file in which this macro exists

    As the current location is inside the src directory,
    this implies that ../HTTP.sublime-syntax is therefore located in the same directory as our Cargo.toml file

    include_str! MACRO

    This reads a file at compile time
    and includes the bytes directly in the binary as a &'static str
    
    This allows to have the benefit of keeping data separate in a file,
    without the cost of having to read it at runtime

    This obviously makes the binary larger, but nonetheless it's a great tool to make that trade-off

    SYNTAX DEFINITION FILES

    The two files are in the sublime syntax format which is what Syntect uses
    
    This format is a YAML file
    which describes how to assign predefined highlight attributes to pieces of text based on regular expressions
    
    The HTTP syntax defined does some highlighting of the version, status and headers
***/

use crate::errors::{Error, HurlResult};
use syntect::highlighting::ThemeSet;
use syntect::parsing::syntax_definition::SyntaxDefinition;
use syntect::parsing::{SyntaxSet, SyntaxSetBuilder};

pub fn build() -> HurlResult<(SyntaxSet, ThemeSet)> {
    let mut builder = SyntaxSetBuilder::new();
    let http_syntax_def = includ_str!("../HTTP.sublime-syntax");
    let def = SyntaxDefinition::load_from_str(http_syntax_def, true, None)
        .map_err(|_| Error::SyntaxLoadError("HTTP"))?;

    builder.add(def);

    let json_syntax_def = include_str!("../JSON.sublime-syntax");
    let json_def = SyntaxDefinition::load_from_str(json_syntax_def, true, None)
        .map_err(|_| Error::SyntaxLoadError("JSON"))?;

    builder.add(json_def);

    let ss = builder.build();
    let ts = ThemeSet::load_defaults();

    Ok((ss, ts))
}