use std::time::Duration;
use std::thread;
use std::io;
fn main(){
    println!("please enter your intensity");
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("failed to read a line");
    let  integer : u32 = i.trim().parse().expect("please correct input");


    println!("please enter random no ");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("failed to read a line");
    let integer1 : u32 = s.trim().parse().expect("please correct input");
 

    workout_genrate(integer, integer1);


    //Clouser Annotation 

     let clouser_v1= |x:i32| ->i32{
     x
     };

// Clouser Capturing the envirnment

let x= 4;
 
  let v3 = |z:i32| z==x;
  assert!(v3(4));




// Moving OwnerShip to Clouer for Move keyword
let vector = vec![1,2,3];

let clouer = move |z| z==vector;

let y = vec![1,2,3];
assert!(clouer(y));




}

// fn simulated_expansive_calculation(intensity: u32)-> u32{
//     println!("calculating slowly");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn workout_genrate(intensity:u32, random_no:u32){
    let expansive_closure = |num| {
        println!("calculating slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    };


    if intensity < 25 {
        println!("today do : {:?} pushups ",expansive_closure(intensity));
        println!("next do : setups {}",expansive_closure(intensity));
    }
    else{
        if random_no == 3 {
            println!("take a break today");
        }
        else{
            println!("today , run for  : {} minutes",expansive_closure(intensity));
        }
    }
    
}



