fn main() {
calcCircumference(3.4);
calcArea(3.9);
largest_number(45,99);
}

fn calcCircumference(r:f64){
    let crfc = 2.0 *3.142*r;
    println!("{}",crfc);

}

fn calcArea(r:f64){
  let r1 =  r*r;
    let arc = 3.142*r1;
    println!("{}",arc);
}


fn largest_number(num1:i32 , num2:i32 ){
  if (num1>num2){
    println!("{} is largest {}",num1,num2);
  }else if(num2 >num1){
    println!("{} is largest {}",num2,num1);
  }else{
    println!("{} is equal to  {}",num1,num2);
  }
}