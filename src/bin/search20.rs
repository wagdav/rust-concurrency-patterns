use rust_concurrency_patterns::search20;

#[tokio::main]
async fn main() {
    let result = search20("rust").await;
    println!("{}", result)
}
