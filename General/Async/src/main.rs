#[tokio::main]
async fn main(){
    let mut handles = vec![];

    for i in 0..5{
        let handle = tokio::spawn(async move{
            my_function().await;
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.await.unwrap(); 
    }
}

async fn my_function() {
    println!("Here is your active async fn");
}