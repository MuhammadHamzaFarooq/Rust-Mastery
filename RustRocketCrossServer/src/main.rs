#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;    // rocket contribute main work for rocket cros ko attch karwana rocket kay code say.
#[macro_use] extern crate lazy_static;      //lazy statics kay zarya hum apna data store karwayn gye/
#[macro_use] extern crate serde_derive;     // this module main work for serialized and deserialized for data . 
extern crate reqwest;
extern crate rocket_cors;
use std::sync::{Arc, Mutex};         // lazy static kay through hum data Arc , mutex kay environment may data ko capture karynge.
use std::collections::HashMap;       // or jo data  capture karna kay bad hashmap may store karwangye
use rocket::State;        // is may hum ya bata rahay hn kya server kon si state may hay
use rocket::http::Method;
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error,Cors,
    CorsOptions,

};

type ID = usize;    // ya hum nay globel varible banya ha.

#[derive(Debug,PartialEq,Eq, Deserialize)]
struct Message{
    id: ID,
    contents: String,
}

fn make_cors()->Cors{
    let allowed_origins = AllowedOrigins::some_exact(&[   
        "http://server-form.surge.sh/",//is may hum nay bata ya hay kay kis kis origin ko allow karana hay .
        "http://rocket-form-rust-server.surge.sh/",
        "http://localhost:8080/html/",
        "http://localhost:8080", 
        "http://0.0.0.0:8080",  


        ]);
     CorsOptions{
        allowed_origins,
        allowed_methods:vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
       allowed_headers:  AllowedHeaders::some(&[
           "Autherization",
           "Accept",
           "Acess-Control-Allow-Origin",
       ]),
       allow_credentials:true,   // yahn hum nay allow credentials kaya hn or hum hay uersname password bhi add karwa saktay hn.
       ..Default::default()   // ya line may kuch is ki apni default settings hun wohi yahn use ki hn default hi.
     }
     .to_cors()   // .to_cors() metho  say is poray ko convert kar daya cros may or return kar daya function ko.
     .expect("Error while building the Cros")    
}
#[get("/")]
fn hello()->JsonValue{
    json!([
        {
          "id":"01",
          "name":"hamza"
        },
        {
            "id":"02",
            "name":"sufyan"
  
        },
        {
            "id":"03",
            "name":"mavia"
  
        }
    ])
}

type MessageMap= Mutex<HashMap<ID,String>>;   // ya hum nay globel varible banya hay message map kay name say or Mutex ko use kar kay hashmap kay under value ko stor kaya ha.
#[post("/add",data="<user_input>")]
fn helloPost(user_input:Json<Message>,map:State<'_,MessageMap>){   //user_input variable Json may value lay ga message struct say , map varible us par iteration perform kary ga.State ka kam hay us jaga say jo client requset karay ga un ko ko apnay server say comnucate karway ga.
    println!("{:?}",user_input.0.contents);  // .0 ka matalb ya ak tuple hay 

}

fn rocket()->rocket::Rocket{
    rocket::ignite().mount("/",routes![hello,helloPost]).attach(make_cors()).manage(Mutex::new(HashMap::<ID,String>::new())) // manage method kay under Mutex ko ku asay define kay hay ku kay hamy code may use karana hay or hum nay make cros kay function may mutex ko use kay hay is lay is ko chala nay kay laya is thara mutex ko manage ka method may define karn gye.
}


fn main(){
rocket().launch();
}