use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("starting service");

    println!("Hello, world!");
}
