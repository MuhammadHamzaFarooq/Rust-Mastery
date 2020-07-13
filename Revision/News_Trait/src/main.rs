#[derive(Debug)]
pub struct NewsArtical{
    pub auther:String,
   pub content:String,

}
#[derive(Debug)]
pub struct Tweet{
    pub username:String,
    pub content:String,
}

pub trait Summary{
    fn summarize(&self)->String{
        format!("read more")
    }

    fn summarize_auther(&self)->String{
        format!("{}",&self.summarize())
    }
}

impl Summary for NewsArtical{
    fn summarize(&self)->String{
        format!("{} says by {}",&self.auther,&self.content)
    }
}

impl Summary for Tweet{
    // fn summarize(&self)->String{
    //     format!("{} tweet by {}",&self.username,&self.content)
    // }
}

fn main(){
    let new = NewsArtical{auther:String::from("Jhon"),content:String::from("Imran Khan Is The New Prime Minister of Pakistan")};
    let tweet = Tweet{username:String::from("Baber Azam"),content:String::from("Michel Stark Is the dangerous boler of the world")};

    println!("{:#?}",new.summarize());
    println!("{:#?}",tweet.summarize());
}
