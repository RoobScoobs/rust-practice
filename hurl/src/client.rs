/***
 * 
 *
    THE MECHANISM FOR MAKING HTTP REQUESTS
***/

use crate::app::{App, Method, Parameter};
use crate::errors::{Error, HurlResult};
use log::{info, debug, trace, log_enabled, self};
use reqwest::multipart::Form;
use reqwest::{Client, RequestBuilder, Response, Url};
use serde_json::Value;
use std::collection::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;