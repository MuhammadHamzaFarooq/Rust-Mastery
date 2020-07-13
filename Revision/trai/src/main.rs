#[derive(Debug)]
struct Point<T,U> {
    x: T,
    y: U,

}
impl<T,U> Point<T,U>{
  fn mixup<V,W>(&self,other:Point<V,W>)-><T,W>{
Point{
    x:&self.x,
    y:other.y,
}
  }
}
fn main(){
    let p = Point{
        x:66,
        y:33.99,
    };

    let p1 = Point{
        x:"hello",
        y: 'a'
    };

    let p2 = p.mixup(p1);


   
}





























































































































// fn main(){
//     let mut a = String::from("hamza");
//   let val = largest(&a);
// }
// fn largest<T>(list : &T)->T{
//     let mut largest =list[0];
//     for &item in list {
//         if item > largest{
//             largest=item;
//         }
//     }
// largest
// }