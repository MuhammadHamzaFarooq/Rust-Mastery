//Completet Async code 
//importing library
use std::time::Duration;
use async_std::task;
use futures::executor::block_on;

fn main(){
 block_on(async_main());
println!("heyyy guys");
}


async fn learn_song()->String{
    task::sleep(Duration::from_secs(5)).await;
    "Tarey Zameen par".to_string()
}

async fn sing_song(song:String){
    task::sleep(Duration::from_secs(3)).await;
    println!("Sing song  :{}",song);
}


async fn learn_sing_and_song(){
    let a = learn_song().await;
    sing_song(a).await;
}

async fn dance(){
    println!("dance!!");
}

async fn async_main(){
    let f1 = learn_sing_and_song();
    let f2 = dance();
    futures::join!(f1,f2);
}



// This code behaviour asychronou and synchronus
// //importing library
// use std::thread;
// use std::time::Duration;
// use futures::executor::block_on;

// fn main(){
// block_on(async_main());
// }


// async fn learn_song()->String{
//     thread::sleep(Duration::from_secs(5));
//     "Tarey Zameen Par".to_string()

// }


// async fn sing_song(song:String){
//     thread::sleep(Duration::from_secs(7));
//     println!("Sing song this : {}",song);    

// }

// async fn sing_and_song(){
//     let song = learn_song().await;
//     sing_song(song).await;
// }

// async fn dance(){
//     println!("Dacne!!");
// }

// async fn async_main(){
//     let f1 = sing_and_song();
//     let f2=dance();
//     futures::join!(f2,f1);

// }




// //learn and sing song application for synchronou and asynchronous environment  //

// // importing  library//

// use std::thread;
// use std::time::Duration;
// use futures::executor::block_on;



// //this synchronou main function
// fn main(){
//  block_on(async_main());
// }

// // this is asyncronou function is functionality is learn song 
// async fn learn_song() -> String {
//     thread::sleep(Duration::from_secs(2));
//     let song_name = String::from("Tarey zameen par");
//     song_name
    

// }


// async fn sing_song(song : String){
//     thread::sleep(Duration::from_secs(2));
//     println!("sing song this : {}",song);
// }


// async fn dance(){
//     println!("dance");
// }


// async fn learn_and_sing(){
//     let song =learn_song().await;// await hum ki bhi asyncronou function ko wait kar wanay kalya use kar tay hn. Ya hum jab kartay hn jab ka kam ya function kisi dosray kam ya function par dependent ho.
//     sing_song(song).await;
// }


// //ya async funtion bnaya hay jis may hum asyncronou functio ko call karyn gay ku kay ap ko to pata hay kay asynchornous function synchornous main() kay function may call nahi ho sakty hn.
// async fn async_main(){
//     let f1 = learn_and_sing();
//     let f2 = dance();
    
//     futures::join!(f1,f2);
// }



























