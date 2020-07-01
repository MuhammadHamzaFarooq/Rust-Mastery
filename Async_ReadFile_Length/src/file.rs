//Importing Library
use async_std::io;
use async_std::fs::File;
use async_std::prelude::*;

//Creating asynchronous function and perform read file task function return Reult enum Ok variant and then calling into main.rs and perform desired task.
pub async fn read_file(path: &str)-> io::Result<String>{
    let mut file = File::open(path).await?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;
    Ok(buffer)
}


