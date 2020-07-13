// Creting Struct for Rectangle //
#[derive(Debug)]
struct Rectangel{
width : u32,
height: u32,
}

//Implement a associated function
impl Rectangel{
    fn area(i:u32)->Rectangel{
        Rectangel{
            width:i,
            height:i,
        }
    }
}
 


fn main(){
  // Creating a instance 
  let rec = Rectangel::area(8);
  print!("{:?}",rec );
}