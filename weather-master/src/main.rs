#![feature(conservative_impl_trait, plugin)]
#![plugin(dotenv_macros)]

#[macro_use] extern crate serde_derive;

extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod command;
mod error;
mod model;
mod rest;

static API_KEY: &str = dotenv!("OWM_API_KEY");

fn main() {
    for query in command::queries() {
        match rest::query(&query) {
            Err(e) => println!("{}", e),
            Ok(result) => {
                println!(
                    "{}: {:.0}°, {}",
                    result.city(),
                    result.temperature(),
                    result.wind()
                )
            }
        }
    }
}
