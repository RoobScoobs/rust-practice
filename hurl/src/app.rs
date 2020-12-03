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

    An Option around a type signifies that argument is optional,
    whereas a non-optional field that is not a bool would be treated as a required positional argument

    The reason to use parse(from_occurrences) on the verbose field is to allow
    the u8 field to be inferred from the number of times the argument is passed
    This is a common pattern to use for verbosity by command line tools

    A special field worth pointing out is cmd,
    which has an attribute of subcommand

    Basically this means that a Method type needs to be defined
    which also derives StructOpt which will be used to create a subprogram here
    As this is wrapped in an Option it's not required

    The Vec<Parameter> type means that multiple values of this type of
    input can be accepted

    As this is a custom type (<Parameter>) parse_param also needs to be implemented in order
    to work with the attribute that allows the definition of a custom parsing function

    APP METHODS

    Create a validate method to check whether a cmd or url exists
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

    /// The parameters for the request if a method subcommand is not specified
    /// 
    /// There are seven types of parameters that can be added to a command-line.
    /// Each type of parameter is distinguished by the unique separator between
    /// the key and value
    /// 
    /// Header -- key:value
    /// 
    ///    e.g. X-API-TOKEN:abc123
    /// 
    /// File upload -- key@filename
    /// 
    ///    This simulates a file upload via multipart/form-data and requires --form
    /// 
    /// Query parameter -- key==value
    /// 
    ///    e.g. foo==bar becomes example.com?foo=bar
    /// 
    /// Data field -- key=value
    /// 
    ///    e.g. foo=bar becomes {"foo": "bar"} for JSON or form encoded
    /// 
    /// Data field from file -- key=@filename
    /// 
    ///    e.g. foo=@bar.txt becomes {"foo": "the contents of bar.txt"} or form encoded
    /// 
    /// Raw JSON data where the value should be parsed to JSON first -- key:=value
    /// 
    ///   e.g. foo:=[1,2,3] becomes {"foo": [1,2,3]}
    /// 
    /// Raw JSON data from file -- key:=@filename
    /// 
    ///   e.g. foo:=@bar.json becomes {"foo": {"bar": "this is from bar.json"}}
    #[structopt(parse(try_from_str = parse_param))]
    pub parameters: Vec<Parameter>,
}

impl App {
    pub fn validate(&mut self) -> HurlResult<()> {
        if self.cmd.is_none() && self.url.is_none() {
            return Err(Error::MisingUrlAndCommand);
        }
        Ok(())
    }
}