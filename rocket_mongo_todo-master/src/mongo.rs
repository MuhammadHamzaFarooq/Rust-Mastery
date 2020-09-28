use mongodb::{
    sync::{
        Client, 
        Collection
    },
    results::{
        DeleteResult
    },
    bson::{
        self,
        doc, 
        Bson::Document,
        oid::ObjectId
    },
    error::Error
};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Todo{
    #[serde(rename = "_id")] 
    pub id:Option<ObjectId>,
    pub item:Option<String>,
    pub status:Option<bool>
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct InsertableTodo{
    pub item:Option<String>,
    pub status:Option<bool>
}

impl InsertableTodo{
    pub fn from_todo(todo:Todo)->InsertableTodo{
        InsertableTodo{
            item:todo.item,
            status:todo.status
        }
    }
}

// collection Name
const TODO:&str = "todo_item";

// Mongo DB Conection
fn mongo_connection(coll:&str)->Result<Collection, Error>{
    // let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let client = Client::with_uri_str("mongodb+srv://monodbUser:abc@myfisrtlerningapp-zbcua.mongodb.net/<dbname>?retryWrites=true&w=majority")?;
    // let client = Client::with_uri_str("mongodb://ubysak1nxbrx6pllwqze:ZFLMw1swQ4KfqBG3OqF4@bnrn7mx78ld977k-mongodb.services.clever-cloud.com:27017/bnrn7mx78ld977k")?;
    let db = client.database("bnrn7mx78ld977k");
    let collection = db.collection(coll);
    Ok(collection)
}


// GET Collection 
fn doc_coll(coll_name:&str)->Collection{
    let collection = match mongo_connection(coll_name){
        Ok(coll) => coll,
        Err(_) => panic!("Error in collection")
    };
    collection
}



// Get All Data
pub fn all()->Result<Vec<Todo>,Error>{
    let collection = doc_coll(TODO);
    let cursor = collection.find(None, None).unwrap();


    cursor.map(|result|match result{
        
    Ok(doc) => match bson::from_bson(Document(doc)) {
        Ok(result_model) => Ok(result_model),
        Err(_) => panic!("document not found"),
    },
    Err(err) => Err(err),
        })
.collect::<Result<Vec<Todo>, Error>>()
}


// Insert One Record
pub fn insert(todo:Todo)->Result<ObjectId,Error>{

    let collection = doc_coll(TODO);
    let insertable = InsertableTodo::from_todo(todo.clone());
    match bson::to_bson(&insertable){
        Ok(model_bson)=>match model_bson {
            Document(model_doc)=>{
                match collection.insert_one(model_doc, None) {
                    Ok(res)=>match res.inserted_id {
                        res=> match bson::from_bson(res) {
                            Ok(res) => Ok(res),
                            Err(_) => panic!("Error"),
                        },
                        _ => panic!("Error")
                    },
                    Err(err) => Err(err)
                }
            }
            _ => panic!("Not inserted")
        },
        Err(_) => panic!("Not Found !!! or inserted")
    }
}

pub fn delete_collection(id: ObjectId)-> Result<DeleteResult, Error>{
    let collection = doc_coll(TODO);
    collection.delete_one(doc! {"_id": id}, None)
}

pub fn update_collection(id: ObjectId, todo: Todo) -> Result<Todo, Error> {
    let collection = doc_coll(TODO);
    let mut new_todo = todo.clone();
    new_todo.id = Some(id.clone());
    match bson::to_bson(&new_todo) {
        Ok(model_bson) => match model_bson {
            Document(model_doc) => {
                match collection.replace_one(doc! {"_id": id}, model_doc, None)
                {
                    Ok(_) => Ok(new_todo),
                    Err(err) => Err(err),
                }
            }
            _ => panic!("Error insert document"),
        },
        Err(_) => panic!("Error insert document"),
    }
}
