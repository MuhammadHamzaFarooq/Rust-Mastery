// importing library
use std::fmt::Display;
fn main(){
    password_checker(String::from("Hello world"));
    password_checker(2);
    password_checker("hamza farooq");
    // creating vector
    let mut v = vec![1,2,3,33,4,5];
    //creating a largest variable
    let mut largest = v[0];

    //for loop
    for item in v {
        if (item> largest) {
        largest = item;
        }
    }
    println!("{}",largest);

    //vector 2
    let mut a = vec![89,98,00,76,56,45,87,56,6363,0,9,0,6,5,7];

let result = largest_fn(&a);
   println!("{}",result);
    
}
 


//creating a function thats check the pasword 
fn password_checker<T:Display>(p: T){
    println!("{}",p);
}



// cater with largest function
fn largest_fn(x : &[i32])->i32{
    //creating a largest variable
    let mut largest = x[0];

    //for loop
    for &item in x.iter() {
        if (item> largest) {
        largest = item;
        }
    }
 largest
   
}