use std::thread;
use std::time::Duration;

fn main(){
    //concurrency  + threading 


    // this is simple code 
    // for i in 1..10{
    //     println!("excuting i loop : {}",i);
    // }


    // for j in 1..10{
    //     println!("excuting j loop : {}",j);
    // }


    // this is thread code

 let handle =   thread::spawn(||{
        for i in 1..10{
            println!("excuting from new thread : {}",i);
            thread::sleep(Duration::from_millis(1));
        }
        
    });

    


    for j in 1..6{
        println!("excuting from main thread : {}",j);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    


}