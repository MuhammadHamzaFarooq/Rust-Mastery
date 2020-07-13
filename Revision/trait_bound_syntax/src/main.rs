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
//trait bound syntax
fn notify<T:Summary>(item:T)->String{
format!("{}",item.summarize())
}


fn main(){
let news=NewsArtical{auther:String::from("Jhon"),content:String::from("It is ranning")};
println!("{:#?}",notify(news));


let tweet= Tweet{username:String::from("Muhammad Hamza Farooq"),content:String::from("Baber Azam is the best player in the pakistan team ")};
let news1=NewsArtical{auther:String::from("Ben Stokes"),content:String::from("It is ranning")};
println!("{:#?}",notify1(tweet,news1));


let tweetUpdate= Tweet{username:String::from("Muhammad Hamza Farooq "),content:String::from("Sir Inzamam malik is best chatbot developer in the world")};
let news1Update=NewsArtical{auther:String::from("Jeff"),content:String::from("It is Holiday")};
println!("{:#?}",notify2(tweetUpdate,news1Update));


}

// implement syntax refactor for tarit bound syntax
fn notify1(item1:impl Summary,item2:impl Summary)->String{
    format!("{} {}",item1.summarize(),item2.summarize()) 
}

//update trait bound syntax
fn notify2<U:Summary,V:Summary>(itemA:U,itemB:V)->String{
    format!("{} {}",itemA.summarize(),itemB.summarize()) 
}