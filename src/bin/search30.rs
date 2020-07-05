use rust_concurrency_patterns::search30;

#[tokio::main]
async fn main() {
    let result = search30("rust").await;
    println!("{}", result)
}
