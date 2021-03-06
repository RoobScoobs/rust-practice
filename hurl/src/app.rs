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

    Add a helper (log_level) to turn the quiet and verbose settings into a string log level for use with the logging implementation

    DATA STRUCTURE FOR THE SUBCOMMAND

    Creating an enum to enable the use the name of the enum, which is an HTTP method, as the name of the subcommand

    Each variant has the same inner data which itself derives StructOpt
    and a helper method is defined to get the data out of each variant

    The one extra attribute in use here is the rename_all = "screaming_snake_case"
    Given the attribute the program uses the form:
        hurl POST whatever.com instead of hurl post whatever.com

    The inner data for each enum variant is a struct to contain the URL and the parameters

    ENUM DEFINITIONS

    Parameter type is used to specify the data for the different types of parameters that accept
    Each one has a particular use case

    Part of this module's aim is to use a parse_param function that will take a string
    and potentially turn it into a Parameter; thus defining a Token type will help with that

    HELPER FUNCTION TO PARSE STRING INTO A VECTOR OF TOKENS

    There are special separator characters like : and == 
    however need a form of escaping to allow those to appear in keys and values

    The gather escapes fn uses a pair of indexes into the source string
    along with possibly some lookahead to tokenize the input

    Basically this is looking for \, =, @, and : following a \ character to indicate that
    this otherwise special character should be escaped and treated as a literal

    THE PARSE PARAM FUNCTION

    It will take a string from the command line
    and attempt to turn it into a Parameter or an appropriate error

    Starts by using the gather_escapes helper function to tokenize the input
    Then loop over those tokens to look for separators

    Trying to find the earliest and longest separator as some of them have overlap

    The found vector will contain all of the separators that match in the first text segment with a separator

    The index is also stored in the segment where this separator starts
    If there's anything in the found list can stop looking for more, ergo the break keyword

    If no separators are found, then error out as that is an erroneous state

    Sort the found list by location and then length of the separator
    Since the vector is not empty can extract the separator for this parameter
    from the first element in the vector

    Following this piece use the data stored so far to construct a key and value for the particular separator

    Finally use the text value of the separator to get a separator type
    which we then use to construct the appropriate Parameter

    ADDING CONFIGURATION TO THE APP

    Bringing in PathBuf for working with file system paths

    The location of the configuration file is something that can optionally be further configured
    hence the added config field to the App struct, which takes a path to look for the file

    The new structopt attribute here is env = "HURL_CONFIG"
    which allows the user to set the location of the configuration file via the HURL_CONFIG environment variable
    in addition to the ability to pass it as a command line argument
    
    The parse(from_os_str) attribute to get the PathBuf is something that is builtin to structopt
    as this is a very common need

    Within the process_config_file use helper functions from the config module to get the path
    and read the file if a path exists

    Then use the resulting data structure, if able to find and parse one, to update the App struct

    SUPPORTING SESSIONS IN THE APP MODULE

    Because the app module is responsible for command line argument based configuration
    the responsibility here is to add the fields to the App struct necessary for configuring the session

    The session field is a name to use for a particular session
    Specifying the same name is how sessions get reused across requests

    The app also provides the ability to specify where the session data should be stored
    By default things are placed in the configuration directory for convenience,
    but the user has the option to put it somewhere else

    Finally, the read_only field determines whether the session should be modified by the request and response
    or if it should only be used to augment the request as it currently exists on disk

    The host method makes use of a helper function make_safe_pathname from the sessions module
    and this is a simple way to get some string representation of the URL that's used to make requests to

    Sessions are unique based on this host value and the configured named

***/

use log::{debug, trace};
use std::convert::TryFrom;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::config;
use crate::errors::{Error, HurlResult};
use crate::session::make_safe_pathname;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "screaming_snake_case")]
pub enum Method {
    HEAD(MethodData),
    GET(MethodData),
    PUT(MethodData),
    POST(MethodData),
    PATCH(MethodData),
    DELETE(MethodData),
}

#[derive(Debug)]
pub enum Parameter {
    // :
    Header {
        key: String,
        value: String
    },
    // =
    Data {
        key: String,
        value: String
    },
    // :=
    RawJsonData {
        key: String,
        value: String
    },
    // ==
    Query {
        key: String,
        value: String
    },
    // @
    FormFile {
        key: String,
        filename: String
    },
    // =@
    DataFile {
        key: String,
        filename: String
    },
    // :=@
    RawJsonDataFile {
        key: String,
        filename: String
    },
}

#[derive(Debug)]
enum Token<'a> {
    Text(&'a str),
    Escape(char),
}

#[derive(Debug)]
enum Separator {
    Colon,
    Equal,
    At,
    ColonEqual,
    EqualEqual,
    EqualAt,
    Snail,
}

#[derive(StructOpt, Debug)]
pub struct MethodData {
    /// The URL to request
    pub url: String,

    /// The headers, data, and query parameters to add to the request.
    #[structopt(parse(try_from_str = parse_param))]
    pub parameters: Vec<Parameter>,
}

/// A command line HTTP client
#[derive(StructOpt, Debug)]
#[structopt(name = "hurl")]
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

    /// Configuration file
    /// 
    /// A TOML file which is stored by default at HOME/.config/hurl/config
    /// where HOME is platform dependent.
    /// 
    /// The file supports the following optional keys with the given types:
    /// verbose: u8
    /// form: bool
    /// auth: string
    /// token: string
    /// secure: bool
    /// 
    /// Each option has the same meaning as the corresponding configuration option with the same name.
    /// The verbose setting is a number from 0 - meaning no logging - to 5 - meaning maximal log output
    #[structopt(short, long, env = "HURL_CONFIG", parse(from_os_str))]
    pub config: Option<PathBuf>,

    /// Session name
    #[structopt(long)]
    pub session: Option<String>,

    /// Session storage
    #[structopt(long, parse(from_os_str))]
    pub session_dir: Option<PathBuf>,

    /// If true then use the stored session to augment the request,
    /// but do not modify what is stored.
    #[structopt(long)]
    pub read_only: bool,
}

impl App {
    pub fn validate(&mut self) -> HurlResult<()> {
        if self.cmd.is_none() && self.url.is_none() {
            return Err(Error::MissingUrlAndCommand);
        }
        Ok(())
    }

    pub fn log_level(&self) -> Option<&'static str> {
        if self.quiet || self.verbose <= 0 {
            return None;
        }

        match self.verbose {
            1 => Some("error"),
            2 => Some("warn"),
            3 => Some("info"),
            4 => Some("debug"),
            _ => Some("trace"),
        }
    }

    pub fn process_config_file(&mut self) {
        let config_path = config::config_file(self);
        let config_opt = config::read_config_file(config_path);
        
        if let Some(mut config) = config_opt {
            if self.verbose == 0 {
                if let Some(v) = config.verbose {
                    self.verbose = v;
                }
            }

            if !self.form {
                if let Some(f) = config.form {
                    self.form = f;
                }
            }

            if !self.secure {
                if let Some(s) = config.secure {
                    self.secure = s;
                }
            }

            if self.auth.is_none() {
                self.auth = config.auth.take();
            }

            if self.token.is_none() {
                self.token = config.token.take();
            }
        }
    }

    pub fn host(&self) -> String {
        if let Some(url) = &self.url {
            make_safe_pathname(url)
        } else if let Some(cmd) = &self.cmd {
            make_safe_pathname(&cmd.data().url)
        } else {
            unreachable!();
        }
    }
}

impl Method {
    pub fn data(&self) -> &MethodData {
        use Method::*;

        match self {
            HEAD(x) => x,
            GET(x) => x,
            PUT(x) => x,
            POST(x) => x,
            PATCH(x) => x,
            DELETE(x) => x,
        }
    }
}

impl From<&Method> for reqwest::Method {
    fn from(m: &Method) -> reqwest::Method {
        match m {
            Method::HEAD(_) => reqwest::Method::HEAD,
            Method::GET(_) => reqwest::Method::GET,
            Method::PUT(_) => reqwest::Method::PUT,
            Method::POST(_) => reqwest::Method::POST,
            Method::PATCH(_) => reqwest::Method::PATCH,
            Method::DELETE(_) => reqwest::Method::DELETE,
        }
    }
}

impl TryFrom<&str> for Separator {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ":" => Ok(Separator::Colon),
            "=" => Ok(Separator::Equal),
            "@" => Ok(Separator::At),
            ":=" => Ok(Separator::ColonEqual),
            "==" => Ok(Separator::EqualEqual),
            "=@" => Ok(Separator::EqualAt),
            ":=@" => Ok(Separator::Snail),
            _ => Err(()),
        }
    }
}

impl Parameter {
    pub fn is_form_file(&self) -> bool {
        match *self {
            Parameter::FormFile { .. } => true,
            _ => false,
        }
    }

    pub fn is_data(&self) -> bool {
        match *self {
            Parameter::Header { .. } => false,
            Parameter::Query { .. } => false,
            _ => true,
        }
    }
}

fn gather_escapes<'a>(src: &'a str) -> Vec<Token<'a>> {
    let mut tokens = Vec::new();
    let mut start = 0;
    let mut end = 0;
    let mut chars = src.chars();

    loop {
        let a = chars.next();

        if a.is_none() {
            if start != end {
                tokens.push(Token::Text(&src[start..end]));
            }
            return tokens;
        }

        let c = a.unwrap();

        if c != '\\' {
            end += 1;
            continue;
        }

        let b = chars.next();

        if b.is_none() {
            tokens.push(Token::Text(&src[start..end + 1]));
            return tokens;
        }

        let c = b.unwrap();

        match c {
            '\\' | '=' | '@' | ':' => {
                if start != end {
                    tokens.push(Token::Text(&src[start..end]));
                }
                tokens.push(Token::Escape(c));
                end += 2;
                start = end;
            }
            _ => end += 2,
        }
    }
}

fn parse_param(src: &str) -> HurlResult<Parameter> {
    debug!("Parsing: {}", src);
    let separators = [":=@", "=@", "==", ":=", "@", "=", ":"];
    let tokens = gather_escapes(src);

    let mut found = Vec::new();
    let mut idx = 0;

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::Text(s) => {
                for sep in separators.iter() {
                    if let Some(n) = s.find(sep) {
                        found.push((n, sep));
                    }
                }
                if !found.is_empty() {
                    idx = i;
                    break;
                }
            }
            Token::Escape(_) => {}
        }
    }

    if found.is_empty() {
        return Err(Error::ParameterMissingSeparator(src.to_owned()));
    }

    found.sort_by(|(ai, asep), (bi, bsep) | ai.cmp(bi).then(bsep.len().cmp(&asep.len())));

    let sep = found.first().unwrap().1;
    trace!("Found separator: {}", sep);

    let mut key = String::new();
    let mut value = String::new();

    for (i, token) in tokens.iter().enumerate() {
        if i < idx {
            match token {
                Token::Text(s) => key.push_str(&s),
                Token::Escape(c) => {
                    key.push('\\');
                    key.push(*c);
                }
            }
        } else if i > idx {
            match token {
                Token::Text(s) => value.push_str(&s),
                Token::Escape(c) => {
                    value.push('\\');
                    value.push(*c);
                }
            }
        } else {
            if let Token::Text(s) = token {
                let parts: Vec<&str> = s.splitn(2, sep).collect();
                let k = parts.first().unwrap();
                let v = parts.last().unwrap();

                key.push_str(k);
                value.push_str(v);
            } else {
                unreachable!();
            }
        }
    }

    if let Ok(separator) = Separator::try_from(*sep) {
        match separator {
            Separator::At => Ok(Parameter::FormFile {
                key,
                filename: value,
            }),
            Separator::Equal => Ok(Parameter::Data {
                key,
                value,
            }),
            Separator::Colon => Ok(Parameter::Header {
                key,
                value
            }),
            Separator::ColonEqual => Ok(Parameter::RawJsonData {
                key,
                value
            }),
            Separator::EqualEqual => Ok(Parameter::Query {
                key,
                value
            }),
            Separator::EqualAt => Ok(Parameter::DataFile {
                key,
                filename: value
            }),
            Separator::Snail => Ok(Parameter::RawJsonDataFile {
                key,
                filename: value
            }),
        }
    } else {
        unreachable!();
    }
}