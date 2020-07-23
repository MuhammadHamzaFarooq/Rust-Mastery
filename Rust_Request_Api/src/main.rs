#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
extern crate reqwest;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

use std::io::prelude::*;
use std::path::Path;
use std::io::Read;
extern crate rustc_serialize;
use rustc_serialize::json::Json;


// web_methods or web_attributes GET,PUT,DELETE,POST
#[get("/")]// this get flag starting on the root dicectory
fn hello()->String{
let path = Path::new("api.json");
    let display = path.display();
    println!("{:?} {:?}",path,display );

    let mut file = match File::create(path){
        Ok(file) => file,
        Err(_)=>panic!("Could not create file "),
    };


    match reqwest::get("http://api.weatherstack.com/current?access_key=c107b8413da3693b8ac3f94263410407&query=Pakistan%20karachi"){
        Ok(mut response )=>{
            match response.text() {
                Ok(text)=> match file.write_all(text.as_bytes()){
                    Ok(_)=>println!("Data write in file "),
                    Err(e)=> println!("The error in this file {}",e),
                }
                Err(_)=>println!("The response is not come from the server"),
}
        }
    Err(_)=>println!("Sever could not established the connection"),
    }

 let mut file = match File::open(&path){
    Ok(file)=>file,
    Err(why)=>panic!("The file open error {}",why.description()),
};

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let json = Json::from_str(&buffer).unwrap();

    let result = format!("The temperature of karachi is this : {}",json.find_path(&["current"]).unwrap());
result
}
fn main(){
    rocket::ignite().mount("/",routes![hello]).launch();
}