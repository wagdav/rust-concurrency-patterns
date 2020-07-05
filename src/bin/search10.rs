use rust_concurrency_patterns::search10;

#[tokio::main]
async fn main() {
    let result = search10("rust").await;
    println!("{}", result)
}
