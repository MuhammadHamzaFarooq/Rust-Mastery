fn main() {
    let x = 23;  // x  is the ower of the value in this scope 
    let y = x;// at this point value of x is copy into y  because x value is u32 stores in stack . Stack no implement in ownership rules.
    println!("{}",x);
    println!("{}",y);

    let a = String::from("Hello world");//  a is the owner of the value
    let b = a;  //

}
//IN rust OwnerShip is best feature .
// Ownership Rules
//Each value in Rust has a variable that’s called its owner.
//There can only be one owner at a time.
//When the owner goes out of scope, the value will be dropped.


