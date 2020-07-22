// use async_std::task;
// use std::collections::HashMap;

// fn main()->Result<(),Box<dyn std::error::Error+ Send+ Sync>>{
//     let resp = reqwest::blocking::get("https://httpbin.org/ip")?
//     .json::<HashMap<String,String>>()?;
//     println!("{:#?}",resp);
//     Ok(())
// }


use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
