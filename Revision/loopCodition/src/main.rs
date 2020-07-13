fn main(){
 let val = String::from("Muhammad Hamza Farooq");
 println!("{}",first_word(&val));
}

fn first_word(x: &String)->usize{
    let bytes = x.as_bytes();
    for (i , &items) in bytes.iter().enumerate(){
        if items == b' '{
            return  i;
        }
    }
    x.len()
}



































































































































































































































































































































// fn main() {
// // let a = String::from("ham   za");
// // println!("{:?}",a.as_bytes());
//     // let val = first_word(&a);
//     // println!("{}",val);
// // let val = dangle();
// // println!("{}",val);

 
// //    let s = String::from("hello");
// //    let r1 = &s;
// //    let r2 = &s;
// //    let r3 = &s;
// //    println!("{:p} {:p} {:p}",r1,r2,r3);





// //   for i in (1..4).rev(){
// //       println!("{}",i);
// //   }












// //    let arr =  ["hello ","hamza","sufyan","ali","jameel"];
 
// //          for i in arr.iter(){
// //              println!("{}",i);
// //          }    
 
 
 
//     //creating array 
// //    let  arr = [ 2,4,57,68,99];

// //    let mut index = 0;

// //    while  index < 5{
// //        println!("{}",arr[index]);
// //        index+=1;
// //    }




//     // let mut  counter = 0;

//     // let result = loop{
//     //     counter +=1;
//     //     if counter ==10{
//     //         break counter *2;

//     //     }
//     // };
//     // println!("{}",result);


//     // let mut number= 4;
//     // while  number == 4{
//     //     println!("{}",number);
//     //     number +=1;
//     // }
// }

// // fn dangle()->&String{
// //     let s = String::from("hello");
    
// //     &s
// // }

// // fn first_word(s: &String) -> usize {
// //     let bytes = s.as_bytes();

// //     for (i, &item) in bytes.iter().enumerate() {
// //         if item == b' ' {
// //             return i;
// //         }
// //     }

// //     s.len()
// // }