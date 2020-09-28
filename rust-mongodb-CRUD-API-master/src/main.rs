
use mongodb::{options::ClientOptions,Client};
use serde_json::{json ,Value};
use bson::doc;

fn main() {
    //build a connection 
    let client= Client::with_uri_str("mongodb://localhost:27017/?readPreference=primary&appname=MongoDB%20Compass%20Community&ssl=false").expect("Failed to build a  connection");
   
    //build a connection with database and collection

    let db = client.database("test1").collection("tblstudent");

    //inserting a document
    let doc = doc!{"user_name":"hamza"};
    let data = db.insert_one(doc,None).unwrap();
    println!("id is : {:#?}",data);

    //find a document
    let doc1=  doc!{"user_name":"hamza"};
    let data = db.find_one(doc1,None).unwrap();
    match data {
        Some(data)=>{
            let data1 :Value= json!(data);
            println!("{}",data);
        },

        None=>println!("No Record Found"),

    }

    //update a document
    let filter = doc!{"user_name":"hamza"};
    let replace =doc!{"user_name":"sufyan"};
    db.find_one_and_replace(filter, replace,None).unwrap();
  
    //for conforming
    let doc3 = doc!{"user_name":"sufyan"};
    let data = db.find_one(doc3, None).unwrap();
    match data{
        Some(data) =>{
            let data1:Value= json!(data);
            println!("{}",data);
        }
        None => println!("No Record Found"),

    }

    //deleting a document
    let query =  doc!{"user_name":"hamza"};
    db.delete_one(query,None);
     //for conforming
     let doc4= doc! { "user_name": "hamza" };
     let data = db.find_one(doc4, None).unwrap();
     match data {
         Some(data) => {
             let data: Value = json!(data);
             println!("data is : {}", data["user_name"]);
         }
         None => println!("No record Found and your desired data is successfully deleted !!!"),
     }
}