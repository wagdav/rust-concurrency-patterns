use rand::Rng;
use std::fmt;
use std::time::Duration;
use tokio::time::delay_for;

#[derive(Debug)]
enum SearchKind {
    Image,
    Video,
    Web,
}

type SearchQuery = str;

pub struct SearchResult(String, String, String);

impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

// https://talks.golang.org/2012/concurrency.slide#43
async fn fake_search(query: &SearchQuery, kind: &SearchKind) -> String {
    let ms = rand::thread_rng().gen_range(1, 100);
    let duration = Duration::from_millis(ms);
    delay_for(duration).await;
    format!("{:?} results for {:?} in {:#?}", kind, query, duration)
}

// Run the Web, Image, and Video searches sequentally
// https://talks.golang.org/2012/concurrency.slide#45
pub async fn search10(query: &SearchQuery) -> SearchResult {
    SearchResult(
        fake_search(query, &SearchKind::Web).await,
        fake_search(query, &SearchKind::Image).await,
        fake_search(query, &SearchKind::Video).await,
    )
}

// Run the Web, Image, and Video searches concurrently, and wait for all results.
// https://talks.golang.org/2012/concurrency.slide#47
pub async fn search20(query: &SearchQuery) -> SearchResult {
    let (web, image, video) = tokio::join!(
        fake_search(query, &SearchKind::Web),
        fake_search(query, &SearchKind::Image),
        fake_search(query, &SearchKind::Video),
    );

    SearchResult(web, image, video)
}
