fn main() {
    let a = gives_ownership();

    println!("{}",a);
    println!("{}",gives_ownership());
   let result = String::from("hello world ");

   let s = take_give_back(result);
   println!("{}",s);
   println!("{}",result);



}

fn gives_ownership()->String{
    let s = String::from("hamza");
    s
}

fn take_give_back(x: String)->String{
    x
}
