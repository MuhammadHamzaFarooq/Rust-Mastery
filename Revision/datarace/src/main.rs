fn main() {
//Dangle refference cater with lifetime
// part 1
       let mut p = String::from("hello");

  { let a = &mut p;

    println!("{}",a);
a.push_str("world");
println!("{}",a);
     }
     

   let b = &mut p;

   println!("{}",b);

  // data race part 2
 let mut s = String::from("Muhammad Hamza");
 let a = &s;
 let b =&s;
 let c = &s;
 println!("{} {} {} ",a,b,c);

 let d = &mut s;
 println!("{}",d);

 d.push_str("Farooq");
 println!("{}",d);


}
