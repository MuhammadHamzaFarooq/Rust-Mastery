use std::io;
fn main(){
    //user input 
    println!("Enter a table ");
    let  mut table = String::new();
    io::stdin().read_line(&mut table).expect("failed to read a line");
    let mut int_table :i64 = table.trim().parse().expect("Please enter correct value");
    
    println!("Enter a table number limit  ");
    let  mut range = String::new();
    io::stdin().read_line(&mut range).expect("failed to read a line");
    let mut int_range :i64 = range.trim().parse().expect("Please enter correct value");
    //shadowig
    int_range = int_range+1;

    // for loop
    for i in 1..int_range{
             println!("{} x {} = {}",int_table,i ,(int_table*i));
    }
      
   
     
}


























