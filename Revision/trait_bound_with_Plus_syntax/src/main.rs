use core::fmt::Display;
#[derive(Debug)]
pub struct Tweet{
  pub  username:String,
  pub  content:String,
}

#[derive(Debug)]
pub struct NewsArtical{
   pub auther:String,
  pub  content:String,
}


pub trait Summary{
    fn summarize(&self)->String;
}
impl Summary for NewsArtical{
    fn summarize(&self)->String{
        format!("{} says {}",&self.auther,&self.content )
    }
}


impl Summary for Tweet{
    fn summarize(&self)->String{
        format!("{} tweet by {}",&self.username,&self.content)
    }
}
//trait bound syntax with + syntax means multiple tarit with attach with +

fn notify<T:Summary +Display>(item:T)->String{
format!("{}",item.summarize())
}


fn main(){
let news=NewsArtical{auther:String::from("Jhon"),content:String::from("It is ranning")};
println!("{:?}",notify(news));


let tweet= Tweet{username:String::from("Muhammad Hamza Farooq"),content:String::from("Baber Azam is the best player in the pakistan team ")};

println!("{:?}",notify1(tweet));


}

// implement syntax refactor for tarit bound syntax
fn notify1(item1:impl Summary+Display)->String{
    format!("{}",item1.summarize() 
}

