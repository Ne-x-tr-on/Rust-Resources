//  enum Result<T,E>{
//         Ok(T),
//         Err(E),
//     }

use std::fs::File;
use std::io::ErrorKind;
fn main(){
     let f = File::open("hello.txt");
    //  let f = File::open("hello.txt").unwrap();

    let f = match f {
        Ok(file)=> file,
        // Err(error) => panic!("Problem Opening the file:\n{}",error),

        Err(error)=> match File::create("hello.txt"){
            Ok(Fc)=> Fc,
            Err(e)=>panic!("Error Creating the file :\n{}",e),
        }
    };

    
}