fn main(){
    //Step 1 creating pointer

    let mut s =String::from("Hello World");
    let a = &s;
    let b= &a;
    let c =&b;
    println!("{} {} {}",a,b,c);
    //mutable pointer concepts
    let mut d = &mut s;
    println!("{}",d);
    d.push_str(" pakistan");
    println!("{}",d);

    //data race concepts ya kay ak wakt or ak scope may mutliple immutable refference to ho skatya hn lakin us kay sat mutable refference nahi ho sakta laken agr scope alag kar dyn to mutable refference create karsakty hn
    //dangle refferece ya ka agr hum ak wakt may do mutable refference bana tay hn jo kay allow nahi hay phir bhi agar banay to compiler error dy ga ku is ki wajaha ha ya ka user agr ak wakt may do log ak hi chz  par write karyn wali bat asa to ho hi nahi sakta or jo hum or immutable us scope may define karawy hn to user us ko to samjh ta hay kay ho kabhi change nahi ho gay us scope to ya to ho raha ya ye error ha .
    //lifetime kay zarya hum dangel refference ko kater kartay hn.

}
