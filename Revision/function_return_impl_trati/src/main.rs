#[derive(Debug)]
struct NewsArtical{
    auther:String,
    content:String,
}

#[derive(Debug)]
struct Tweet{
    username:String,
    content:String,

}

pub trait Summary{
    fn summarize(&self)->String;
}

impl Summary for Tweet{
    fn summarize(&self)->String{
        format!("{} tweet by {}",&self.username,&self.content)
    }
}

impl Summary for NewsArtical{
    fn summarize(&self)->String{
        format!("{} says {} ",&self.auther,&self.content)
    }
}
//this function return impl summary trait and funtion return one type
fn create_summary()->impl Summary {
    Tweet{
        username:String::from("Jhon"),
        content:String::from("Today is Holiday")
    }
}


fn main(){
create_summary();
 
   
}