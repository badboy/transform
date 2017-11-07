extern crate serde;
extern crate serde_json as json;
extern crate serde_yaml as yaml;
extern crate toml;

use std::env;
use std::io::{self, Read};
use std::fs::File;

fn main() {
    let mut args = env::args().skip(1);
    let from = args.next().expect("No from given");
    let to = args.next().expect("No to given");
    let input = args.next().expect("No input given");

    let mut input : Box<Read> = match &*input {
        "-" => Box::new(io::stdin()),
        f @ _ => {
            let f = File::open(f).expect("Can't open file");
            Box::new(f)
        }
    };

    let mut data = vec![];
    input.read_to_end(&mut data).expect("Can't read input");
    let data = String::from_utf8(data).expect("Input not valid UTF-8");

    let output = match (&from[..], &to[..]) {
        ("toml", "yaml") => {
            let value : yaml::Value = toml::decode_str(&data).unwrap();
            yaml::to_string(&value).expect("Can't encode input as YAML")
        }
        ("toml", "json") => {
            let value : yaml::Value = toml::decode_str(&data).unwrap();
            json::to_string_pretty(&value).expect("Can't encode input as YAML")
        }
        ("yaml", "toml") => {
            let value : toml::Value = yaml::from_str(&data).unwrap();
            format!("{}", value)
        }
        ("yaml", "json") => {
            let value : yaml::Value = yaml::from_str(&data).unwrap();
            json::to_string_pretty(&value).expect("Can't encode input as YAML")
        }
        ("json", "toml") => {
            let value : toml::Value = json::from_str(&data).unwrap();
            format!("{}", value)
        }
        ("json", "yaml") => {
            let value : yaml::Value = json::from_str(&data).unwrap();
            yaml::to_string(&value).expect("Can't encode input as YAML")
        }
        a @ _ => panic!("Can't go from {} to {}", a.0, a.1),
    };

    println!("{}", output);
}
