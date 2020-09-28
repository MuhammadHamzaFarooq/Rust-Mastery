#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_cors;
use rocket::http::Method; 
use rocket_cors::{
    AllowedHeaders, AllowedOrigins,
    Cors, CorsOptions 
};
use rocket_contrib::json::Json;
mod mongo;
use mongo::{all,insert,update_collection,delete_collection,Todo};
use mongodb::{bson::{doc,oid::ObjectId},error::Error};


 
fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[     
        "http://localhost:8080",
        "http://0.0.0.0:8080",
        "http://rust-react-todo-app.surge.sh/"
    ]);

    CorsOptions { 
        allowed_origins,
        allowed_methods: vec![Method::Get,Method::Post,Method::Delete,Method::Put].into_iter().map(From::from).collect(), 
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
            "Access-Control-Allow-Headers", 
            "Access-Control-*",
            "Origin", 
            "X-Requested-With", 
            "Content-Type", 
            "Accept"
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}



#[get("/")]
fn func_get ()->Result<Json<Vec<Todo>>, Error>{
    match all(){
        Ok(res) => Ok(Json(res)),
        Err(err) => panic!("{}Document not found",err),
    }
}


#[post("/add" ,format = "application/json", data = "<todo>")]
fn func_post (todo:Json<Todo>)-> Result<Json<ObjectId>, Error> {
    match insert(todo.into_inner()) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(err.into()),
    }
}

#[delete("/<id>")]
fn func_delete(id: String) -> Result<Json<String>, Error> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match delete_collection(res) {
            Ok(_) => Ok(Json(id)),
            Err(err) => panic!("{}",err),
        }
        Err(_) => panic!("Cannot parse id")
    }
}


#[put("/<id>", format = "application/json", data = "<todo>")]
fn func_update(id:String,todo:Json<Todo>)->Result<Json<Todo>, Error> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match update_collection(res, todo.into_inner()) {
            Ok(res) => Ok(Json(res)),
            Err(err) => Err(err.into()),
        }
        Err(e) => panic!("Error update {}",e)
    }
}



fn rocket()-> rocket::Rocket{
    rocket::ignite().mount("/", routes![func_get,func_post,func_delete,func_update]).attach(make_cors())
}
fn main() {
    rocket().launch();
}
