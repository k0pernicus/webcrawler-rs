extern crate argparse;
extern crate core;
extern crate curl;
extern crate regex;
extern crate xml;

mod href_parser;

use argparse::{ArgumentParser, StoreTrue, Store};

use curl::http;

use href_parser::ParsedTree;

use std::collections::HashMap;
use std::str::from_utf8;


fn main() {

    let mut refs : Vec<String> = Vec::new();
    let mut u_refs : HashMap<String, u8> = HashMap::new();

    // Arguments
    let mut url : String = String::new();
    let mut verbose = false;

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("A web crawler, in Rust.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Be verbose");
        ap.refer(&mut url)
            .add_option(&["--url"], Store,
            "The URL to begin.");
        ap.parse_args_or_exit();
    }

    let mut pt = ParsedTree::new(&url);

    refs.push(url.to_string());
    u_refs.insert(url.to_string(), 1);

    while !refs.is_empty() {

        let url = refs.pop().unwrap();

        let resp = http::handle()
            .get(url.clone())
            .exec();

        match resp {
            Ok(resp) => {
                let body = from_utf8(resp.get_body());

                match body {
                    Ok(xml_structure) => {
                        pt.run(xml_structure);

                        for url in pt.get_external_references() {
                            if !u_refs.contains_key(&url) {
                                refs.push(url.clone());
                                u_refs.insert(url.clone(), 1);
                                println!("New URL : {}", url);
                            }
                        }
                    },
                    Err(error) => println!("Failed to load XML structure cause of {}", error),
                }
            }
            Err(error) => println!("Error to get response from {} - error: {}", url, error),
        }
    }
}
