//creatin struct for point
struct Point<T,U>{
  x: T,
  y: U,
}
// implimentation for point
impl <T,U> Point <T,U>{

 fn mixup<V,W>(&self , others : Point<V,W>)-><T,W>{
        Point{
         x:&self.x,
         y:others.y,
        }
    }
}
//creating a instances
fn main(){
 let p1 = Point{x:22,y:"Sufyan"};
  let p2 = Point{x:9393.4,y:String::from("Muhammad Hamza Farooq")};
  let p3 = p1.mixup(p2);
}
