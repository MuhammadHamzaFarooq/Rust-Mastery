// //Refrrencing 
// fn main(){
//   let s = String::from("Muhammad Hamza Farooq");
//   let (res, res1)= calculating_lenght(&s);
//   println!("MY Name is : {}  and length is {}",res,res1);

// }

// fn calculating_lenght(x: &String)->(String,usize){
//     (x.to_string(),x.len())
// }

//Refactor code 
// fn main(){
// let s = String::from("hello world");
// let res = calculating_lenght(&s);
// println!("{} length is {}",s, res);

// }

// fn calculating_lenght(x:&String)->usize{
//   x.len()
// }


//mutabel refference 
fn main(){
 let mut s = String::from("Hello ");
 change(&mut s);
}
fn change(x : &mut String){
    x.push_str(" , World");
 println!("{:?}",x ) ; 
}

















//creating pointer
// fn main() {
//     let a = 10;
//     let b = &a;
//     let c = &b;
//     println!("{}",a);
//     println!("{}",b);
//     println!("{}",c);
//     println!("{:p}",b);
//     println!("{:p}",c);
   
// }