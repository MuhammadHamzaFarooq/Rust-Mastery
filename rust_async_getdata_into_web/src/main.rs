
//Importing Library//
use futures::task;
use futures;
use futures::executor::block_on;
use surf;

fn main(){
    block_on(exce());
}
//this async fucntion fetching data into web
async fn fetch(url:&str)->Result<String,surf::Exception>{
  surf::get(url).recv_string().await
}

async fn exce(){
    match fetch("https://www.facebook.com").await{
    Ok(res)=>println!(" {}",res),
    Err(e)=>println!(" {}",e),  
    }
}
