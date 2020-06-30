extern crate rand;

use rand::Rng;
use std::thread;
use std::time;

#[derive(Debug)]
enum SearchKind {
    Image,
    Video,
    Web,
}

type SearchQuery = &'static str;

// https://talks.golang.org/2012/concurrency.slide#43
fn fake_search(query: SearchQuery, kind: SearchKind) {
    let mut rng = rand::thread_rng();
    let delay = time::Duration::from_millis(rng.gen_range(1, 100));
    thread::sleep(delay);
    println!("{:?} results for {:?} in {:#?}", kind, query, delay);
}

// Run the Web, Image, and Video searches sequentally
// https://talks.golang.org/2012/concurrency.slide#45
fn search10(query: SearchQuery) {
    use SearchKind::*;

    for kind in vec![Web, Image, Video].into_iter() {
        fake_search(query, kind);
    }
}

fn main() {
    search10("rust")
}
