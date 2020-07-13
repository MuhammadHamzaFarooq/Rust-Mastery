// struct for super hero
#[derive(Debug)]
struct SpiderMan{
    name:String,
}

#[derive(Debug)]
struct BatMan{
    name:String,
}

#[derive(Debug)]
struct SuperMan{
    name:String,
}

#[derive(Debug)]
struct Hulk{
    name:String,
}

pub trait Power{
    fn power_scores(&self)->u8{
    50
    }
    
}

impl Power for SpiderMan{
    fn power_scores(&self)->u8{
        100
    }
}
impl Power for SuperMan{
    fn power_scores(&self)->u8{
        99
    }
    
}
impl Power for BatMan{
    fn power_scores(&self)->u8{
        95
    }
}

impl Power for Hulk{}

fn main() {
    let spiderman = SpiderMan{name:String::from("SPIDER MAN")};
    let superman = SuperMan{name:String::from("SUPER MAN")};
    let batman = BatMan{name:String::from("BAT MAN")};
    let hulk = Hulk{name:String::from("HULK")};
 println!("{:#?}",spiderman.power_scores());
 println!("{:#?}",superman.power_scores());
println!("{:#?}",hulk.power_scores());
println!("{:#?}",batman.power_scores());
}
