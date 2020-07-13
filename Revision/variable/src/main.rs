//this is main function 
fn main() {
    //declear  a vriable  with data type . In rust  by default data of varibale is i32;
  let a :i32 ;
  // initilize a variable . In rust initiliza varible with value rust infer variable data type automatically.
  let b = String::from("Hello world");
 // IN rust two type of variabe .One is immutable variable this varible value cannot be change in for this variable particular scope . And second is mutable variable this variable value  is change.
 let c = "hello world "; // this immutable variable and data type string scilice . this type of string store in stack.

 let mut d = 44.4; // this is mutable variable and value is change . And variable data type is float . declear mutable variable let keyword and mut keyword.
  d = 66.4;//the value is change 
  println!("{}",d); //resut is 66.4

  //Shadowing in rust
  let i = 3;
  let i = i +2;
  let i = i*6;
  // In rust shadowing is the famous feature 
  println!("{}",i);// this stage i result is ? anwser for audience 

  let j = true; // this variable data type is boolean . boolean data two vaue is true or false 
  println!("{}",j);

  let  val = String::from("My name is hamza");
  println!("{}",val.len()); // len() is the method . this method calculate a length of string 

   // Constant
   const  Max_VAL : i32= 88;  // constant value cannot be change  and varibel name is capital letter.
   println!("{}",Max_VAL);

   // In Rust there and two data type of groups sclar data type and compuand data tpye
   // Scalar data type lis 
   //  i32 , u32 ,i64,f32,f64,bool,usize ,i8,i16 
   //Compound data type list 
   // tupel , arrays

   // Compound type 
   let tup = ("hello",String::from("hamza"),777,292.00); // tuple is compund data type is store multiple data type values
   println!("{:#?}",tup);// tup is not implement a format trait . tuple is impliment a debug trait . tarit is rust featur let dicuses in later 
   
   // array
   let arr = ["hello ","hamza "]; // In rust array is compund data type store mutliple value in multiple index but data the date type in same 
   println!("{:#?}",arr);// array is not  implement a format trait . tuple is impliment a debug trait . tarit is rust featur let dicuses in later 

   //mutli dimensional array
  let multi_arr = [[1,2,3],[4,5,6],[7,8,9]];
  println!("{:?}",multi_arr[0]);// acess value with index no 
  println!("{:?}",multi_arr[1]);// acess value with index no 
  println!("{:?}",multi_arr[2]);// acess value with index no 
    
}

