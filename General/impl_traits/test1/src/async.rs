impl Dad {
    async fn new_async(name: &str) -> Self {
        // pretend we got details from an API
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        Self {
            age: 55,
            name: name.to_string(),
            alive: true,
        }
    }
}


#[tokio::main]
async fn main() {
    let d = Dad::new_async("Kamau").await;
}
