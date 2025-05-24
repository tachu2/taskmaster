mod server;

#[tokio::main]
async fn main() {
    server::serve().await;
    println!("Hello, world!");
}
