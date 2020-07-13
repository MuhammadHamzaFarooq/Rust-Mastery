fn main() {
    let f = Option::Some(88);
    println!("{:?}",f);

}
#[derive(Debug)]
enum Option<T>{
  Some(T),
  None,
}

#[derive(Debug)]
enum Result<T,E>{
    OK(T),
    Err(E)
}