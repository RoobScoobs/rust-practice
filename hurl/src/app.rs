/***
 * 
 * 
    THE APPLICATION MODULE

    This module is the core of how to interact with the user via command line arguments

    The structopt crate is built on the clap crate
    May also see people using Clap to create command line applications

    The structOpt is a type of macro known as a custom derive
    which means that code in the structopt crate will be given the App struct definition as input
    and will output code which will then be included in the crate

    Doc comments are included as part of the struct definition to the custom derive
    and therefore structopt uses the doc comment on this struct as part of the help message
    which gets created as part of the code that is generated

    OPTIONS TO ACCEPT AT THE COMMAND LINE

    The types of the variables determine what type of flag/option is created

    For example, a field with type bool like quiet creates a flag
    which sets the field to true if it is present and is otherwise set to false

    The attributes on the fields are further used for customization
    For example, short, long says that the field should be usable
    using a short form like -q for quiet and a long form --quiet
***/

use log::{debug, trace};
use std::convert::TryFrom;
use structopt::StructOpt;

use crate::errors::{Error, HurlResult};

/// A command line HTTP client
#[derive(StructOpt, Debug)]
#[structopt(name = "hurl"))]
pub struct App {
/// Activate quiet mode
/// 
/// This overrides any verbose settings.
#[structopt(short, long)]
pub quiet: bool,

/// Verbose mode (-v, -vv, -vvv, etc.)
#[structopt(short, long, parse(from_occurrences))]
pub verbose: u8,

/// Form mode
#[structopt(short, long)]
pub form: bool,

/// Basic authentication
/// 
/// A string of the form `username:password`.
/// If only `username` is given, then you will be prompted
/// for a password. If you wish to use no password
/// then use the form `username:`.
#[structopt(short, long)]
pub auth: Option<String>,

/// Bearer token authenication
/// 
/// A token which will be sent as "Bearer <token>"
/// in the authorization header.
#[structopt(short, long)]
pub token: Option<String>,

/// Default transport
/// 
/// If a URL is given without a transport, i.e. example.com/foo
/// http will be used as the transport by default.
/// If this flag is set then https will be used instead.
#[structopt(short, long)]
pub secure: bool,

/// The HTTP Method to use, one of:
/// HEAD, GET, POST, PUT, PATCH, DELETE.
#[structopt(subcommand)]
pub cmd: Option<Method>,

/// The URL to issue a request to
/// if a method subcommand is not specified.
pub url: Option<String>,
}