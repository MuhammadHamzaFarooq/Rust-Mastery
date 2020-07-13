//importing standard library for collection module
use std::collections::HashMap;
use std::collections::*;
fn main(){

   let text = "Muhammad Hamza Farooq";

let mut map = HashMap::new();

for word in text.split_whitespace(){
    let count  = map.entry(word).or_insert(0);
    *count +=1;
} 
println!("{:#?}",map);

























    // let text = "hello world wonderful world";

    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    // println!("{:?}", map);
   
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{:?}", scores);




























  
//     let mut team = vec![String::from("Pakistan "),String::from("India ")];
    
//     let mut initial_scores = vec![50,30];
//     let scores :HashMap<_,_>= team.into_iter().zip(initial_scores.into_iter()).collect();

// println!("{:#?}",scores);








































































































    // let mut  map = HashMap::new();
// map.insert("karachi kings", 10);
// map.insert( "Lahore Qulander", 20);
// println!("{:?}",map);
// let mut s = String::from("hello");

// let data = "world";
// data.to_string();
// s.push_str(data);
// println!("{}",s);
    //  let mut  v :Vec<i32> =Vec::new();
//  v.push(9);
// println!("{:?}",v);
// v.push(10);
// v.pop();
// println!("{:?}",v);



//    let v = vec![1,22,34,55,66,66,45,35,358];
//    let  third =&v[3];
//    match v.get(3) {
//        Some(val)=>{
//            println!("this index value is : {:?}",val);
//        },
//        None=>println!("There is no element"),
//    }


// let mut v = vec![100, 32, 57];
// for i in &mut v {
//     *i += 50;
//     println!("{:?}",i);
// }
}
