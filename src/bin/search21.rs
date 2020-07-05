use rust_concurrency_patterns::search21;

#[tokio::main]
async fn main() {
    let result = search21("rust").await;
    println!("{}", result)
}
