use std::time::Duration;
use std::thread;
use futures::executor::block_on;

async fn get_two_site_async(){
   let thread_one = t1();
   let thread_two = t2();
   
   futures::join!(t1(),t2());
}


async fn t1(){
    thread::sleep(Duration::from_secs(5));
            println!("thread one ");
}

async fn t2(){
    thread::sleep(Duration::from_secs(5));
            println!("thread two ");
}

fn main(){
    block_on(get_two_site_async());

}







































// fn get_two_site(){
//     let thread_one = thread::spawn(||{
     
//         thread::sleep(Duration::from_secs(5));
//         println!("thread one ");
//     });

//     let thread_two = thread::spawn(||{
        
//         thread::sleep(Duration::from_secs(7));
//         println!("thread two ");
//     });


//     thread_one.join();
//     thread_two.join();
// }

// fn main(){
//     get_two_site();
// }