//Importing library
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main(){

    let val = read_username_from_file();
    println!("{:?}",val);
   
      let  f = File::open("src/t1.txt");
      
      let f = match f{
          Ok(file)=>file,
          Err(error)=> match error.kind(){
              ErrorKind::NotFound=> match File::create("src/t1.txt"){
                  Ok(fc) =>fc,
                  Err(e)=>panic!("problem for creating file : {:?}",e),
                  
              }
              other_error=>{
                  panic!("problem for opening file : {:?}",other_error)
              },

          },
      };
      
    let  f = File::open("src/hello.txt");
    let f = match f {
        Ok(file)=>file,
        Err(error)=> match error.kind(){
            ErrorKind::NotFound=> match File::create("src/hello.txt"){
                Ok(fc)=>fc,
                Err(e)=>panic!("problem for creating fiel : {}",e),
            },
            other_error =>{ panic!("problem opening the file {:?}",other_error)}
        },
    };
}


fn read_username_from_file()-> Result<String,io::Error>{
  let mut f = File::open("src/world.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}