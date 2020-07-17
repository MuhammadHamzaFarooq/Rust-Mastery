//importing Library
extern crate reqwest;
 

// synchornous code and multi-threaded code 
fn main(){
    match reqwest::get("https://httpbin.org/ip"){
     Ok(mut res) => {
    match res.text(){
        Ok(text)=>println!(" {}",text),
        Err(_)=> println!("The Error")
    }
}
     Err(_)=> println!("The Error")
    }
}