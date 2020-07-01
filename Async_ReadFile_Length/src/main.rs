//Importing Library
use futures;
use futures::executor::block_on;
use std::io;

mod file; // load coustom file code =>file.rs
fn main() {
    block_on(load_data());
}

async fn load_f1(){
    let result1 = file::read_file("src/t1.txt").await;
    println!("file1 : {}",result1.unwrap().len());

}

async fn load_f2(){
    let result2 = file::read_file("src/t2.txt").await;
    println!("file2 : {}",result2.unwrap().len());    
}

async fn load_data(){
    futures::join!(load_f1(),load_f2());
}
