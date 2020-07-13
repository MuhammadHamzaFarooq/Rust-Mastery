#[derive(Debug)]
enum Coin{
    Penny,
    Dime,
    Nickel,
    Quater(UsState),
}

#[derive(Debug)]
enum UsState{
    Alaska,
    Alabama,
}
fn cents_coin(coin:Coin)->u8{
    match coin{
        Coin::Penny=>1,
        Coin::Dime=>5,
        Coin::Nickel=>6,
        Coin::Quater(state)=>{
            println!("State Quater forum {:#?}",state);
            25
        },
    }
}

fn main(){
 let coin_val = cents_coin(Coin::Quater(UsState::Alabama));
 println!("{:?}",coin_val);
}
















































































































































































































// #[derive(Debug)]
// enum Message{
//     Quit,
//     Move{x:i32,y:i32},
//     Write(String),
//     ChangeColour(i32,i32,i32),
// }
// impl Message{
//     fn call(&self){
//         println!("{:?}",&self);
//     }
// }

// fn main(){
//     let val = Message::Write(String::from("Hello world"));
//     val.call();
// }









































//creating enum 
// #[derive(Debug)]
// enum Coin{
//     Penny,
//     Dime,
//     Nickel,
//     Quater,
// }

// fn cents_coin(coin : Coin)->u8{
//     match coin{
//         Coin::Penny=>1,
//         Coin::Dime=>5,
//         Coin::Nickel=>15,
//         Coin::Quater=>25
//     }
// }

// fn main(){
//     let val =cents_coin(Coin::Penny);
//     println!("{:#?}",val);
// }