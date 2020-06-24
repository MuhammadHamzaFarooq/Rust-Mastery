use std::io;
use RustCalculator_Library::arthimatic_operation::*;

fn main()
{
    println!("----------------Welcome in the simple integer calculator ------------------------ ");
    // User input Code Chunk;
    println!("please enter a first integer number");
      let mut input1=String::new();
      io::stdin().read_line(&mut input1).expect("failed to read a line");
      let int_input1 :i64 = input1.trim().parse().expect("Please enter correct value");

    println!("please enter a second integer number");
      let mut input2=String::new();
      io::stdin().read_line(&mut input2).expect("failed to read a line");
      let int_input2 :i64 = input2.trim().parse().expect("Please enter correct value");
 
      // Calling moduls and function
      //1 - Calling add funtion
      let sum =add(int_input1, int_input2);
      println!("sum is : {}",sum);

      //2 - Calling sub function
      let minus = sub(int_input1,int_input2);
      println!("subtraction is : {}",minus);

      //3 - Calling multi function
      let product = multi(int_input1,int_input2);
      println!("Multiplication is : {}",product);

      //4 - Calling divi function
      let div : f64 = divi(int_input1,int_input2);
      println!("division is : {}",div);

      //5-Calling moduls function
      let reminder = moduls(int_input1,int_input2);
      println!("reminder is : {}",reminder);


}




