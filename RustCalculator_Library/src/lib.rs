pub use crate::arthimatic_operation::add;
pub use crate::arthimatic_operation::sub;
pub use crate::arthimatic_operation::divi;
pub use crate::arthimatic_operation::multi;
pub use crate::arthimatic_operation::moduls;
pub use crate::arthimatic_operation::power_functions::cube;
pub use crate::arthimatic_operation::power_functions::square;
pub use crate::arthimatic_operation::power_functions::pow;


pub mod arthimatic_operation{
    pub fn add(input1: i64 , input2 : i64)->i64{
          input1+input2
    }
    pub fn sub(input1: i64 , input2 : i64)->i64{
        input1-input2
    }
    pub fn divi(input1: i64 , input2 : i64)->f64{
        let x = input1 as f64;
        let y = input2 as f64;
        x/y

        
    }

    pub fn multi(input1: i64 , input2 : i64)->i64{
        input1*input2
    }
    pub fn moduls(input1: i64 , input2 : i64)->i64{
        input1%input2
    }


    pub mod power_functions {
        pub fn square(input1:i64)->i64 {
            input1*input1
        }

        pub fn cube(input1:i64)->i64 {
            input1*input1*input1
        }

        pub fn pow(num : u32,exp: u32)->u32 {
          

            u32::pow(num,exp)
        }
    }


}


