// #[tokio::main]

// async fn main(){
//   let f = my_function().await;
//   f.await;
// }

// // fn my_function() -> impl Future<Output = ()> {
// //   println!("An async function")
// // }

// async fn my_function(){
//   println!("I'm an async function");
//   let s1:String = read_from_database().await;
//   println!("First reult: {s1}");
//   let s2:String = read_from_database().await;
//   println!("Second result: {s2}");
// }

// async fn read_from_database() -> String{
//   "DB result".to_owned()
// }

//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! ```

#[tokio::main]
async fn main() {
    my_function().await;
}

async fn my_function() {
    println!("I'm an async function");

    let s1 = read_from_database().await;
    println!("First result: {}", s1);

    let s2 = read_from_database().await;
    println!("Second result: {}", s2);
}

async fn read_from_database() -> String {
    "DB result".to_owned()
}

