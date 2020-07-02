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

type SearchQuery = str;

// https://talks.golang.org/2012/concurrency.slide#43
fn fake_search(query: &SearchQuery, kind: &SearchKind) {
    let ms = rand::thread_rng().gen_range(1, 100);
    let delay = time::Duration::from_millis(ms);
    thread::sleep(delay);
    println!("{:?} results for {:?} in {:#?}", kind, query, delay);
}

// Run the Web, Image, and Video searches sequentally
// https://talks.golang.org/2012/concurrency.slide#45
fn search10(query: &SearchQuery) {
    use SearchKind::*;

    for kind in vec![Web, Image, Video].iter() {
        fake_search(query, kind);
    }
}

fn main() {
    search10("rust")
}
