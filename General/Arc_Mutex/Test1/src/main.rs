use tokio;

#[tokio::main]
async fn main() {
    let name = String::from("Newton Kamau");

    tokio::spawn(async move {
        println!("Hello from inside a Tokio task, {}!", name);
    });

    // Give time for the spawned task to run
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
