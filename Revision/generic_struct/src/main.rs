fn main() {
 let p1 = Point{x:88,y:77.9};
 println!("{:#?}",p1.Cordinate());
 let p2 = Point{x:39.0,y:3.5};
 println!("{:#?}",p2.distance());
 
}
#[derive(Debug)]
struct Point<U,T>{
 x:U,
 y:T, 
}
impl<U,T> Point <U,T>{
    fn Cordinate(&self)->&U{

        &self.x
    }
   
}

impl Point<f64,f64>{
    fn distance(&self)->f64{
        (&self.x.powi(2)+&self.y.powi(2)).sqrt()
    }
}







