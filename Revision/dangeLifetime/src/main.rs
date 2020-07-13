fn main() {
    let s = String::from("hello");
    let val = dangle(&s);
    println!("{}",val);
    
}

//Dangle cater with lifetime 
fn dangle<'a>(x : &'a String)->&'a String{
   
    x
}