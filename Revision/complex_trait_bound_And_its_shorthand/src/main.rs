use std::fmt::Display;
use std::fmt::Debug;
//complex trait bound syntax
fn notify<T:Display+Clone,U:Clone+Debug>(item1:T,item2:U){

}

//cleaner tarit bound syntax with easier syntax with where keyword
fn notify1<T, U>(itme1: T, item2: U)
where T: Display + Clone,
      U: Clone + Debug{}
fn main(){}

