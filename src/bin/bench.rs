use rust_concurrency_patterns::*;
use std::future::Future;
use std::time::{Duration, Instant};

pub async fn time<F, T>(f: F) -> Duration
where
    F: FnOnce() -> T,
    T: Future<Output = SearchResult>,
    F: Copy,
{
    let now = Instant::now();
    loop {
        print!(".");
        if f().await.succeeded() {
            return now.elapsed();
        }
    }
}

#[tokio::main]
async fn main() {
    println!("search10 in {:?}", time(|| search10("rust")).await);
    println!("search20 in {:?}", time(|| search20("rust")).await);
    println!("search21 in {:?}", time(|| search21("rust")).await);
    println!("search30 in {:?}", time(|| search30("rust")).await);
}
