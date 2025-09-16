pub mod async_;
use crate::async_::get_example;
use crate::async_::post_example;

#[tokio::main]
async fn main(){
    if let Err(e) = get_example().await{
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

