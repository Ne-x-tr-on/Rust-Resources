#[tokio::main]
#[allow(unused)]

async fn main(){
let data = fetch_books();
data.await;
}

async fn fetch_books(){
    let _books = println!("Here are your books");
}