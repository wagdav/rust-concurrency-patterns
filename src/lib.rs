use rand::Rng;
use std::thread;
use std::time;

use rayon::prelude::*;

#[derive(Debug)]
enum SearchKind {
    Image,
    Video,
    Web,
}

type SearchQuery = str;

// https://talks.golang.org/2012/concurrency.slide#43
fn fake_search(query: &SearchQuery, kind: &SearchKind) -> String {
    let ms = rand::thread_rng().gen_range(1, 100);
    let delay = time::Duration::from_millis(ms);
    thread::sleep(delay);
    format!("{:?} results for {:?} in {:#?}", kind, query, delay)
}

pub fn print_result(results: &Vec<String>) {
    println!("{:?}", results)
}

// Run the Web, Image, and Video searches sequentally
// https://talks.golang.org/2012/concurrency.slide#45
pub fn search10(query: &SearchQuery) -> Vec<String> {
    use SearchKind::*;

    [Web, Image, Video]
        .iter()
        .map(|kind| fake_search(query, kind))
        .collect()
}

// Run the Web, Image, and Video searches concurrently, and wait for all results.
// https://talks.golang.org/2012/concurrency.slide#47
pub fn search20(query: &SearchQuery) -> Vec<String> {
    use SearchKind::*;

    [Web, Image, Video]
        .par_iter()
        .map(|kind| fake_search(query, kind))
        .collect()
}
