pub mod async_;
pub mod simple_send;

use crate::simple_send::post_example;
// use crate::async_::get_example;
// use crate::async_::post_example;

#[tokio::main]
async fn main(){
    if let Err(e) = post_example().await{
        eprintln!("Error: {}",e);
    }

    let result = post_example().await;
    match result{
        Ok(_) =>{}
        Err(e) => {
            eprintln!("Error \n{:#?}",e);
        }
    }
}

// let result = get_example().await;
// match result{
//     Ok(_){}
//     Err(e){
//         eprintln!("Error: {}",e);
//     }
// }

