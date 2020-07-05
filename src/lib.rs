use rand::Rng;
use std::fmt;
use std::time::Duration;
use tokio::time::{delay_for, timeout};

#[derive(Debug)]
enum SearchKind {
    Image,
    Video,
    Web,
}

type SearchQuery = str;

#[derive(Debug)]
pub struct SearchResult(Option<(String, String, String)>);

impl SearchResult {
    fn new(a: String, b: String, c: String) -> SearchResult {
        SearchResult(Some((a, b, c)))
    }

    fn timeout() -> SearchResult {
        SearchResult(None)
    }
}

impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self(Some((a, b, c))) => write!(f, "({}, {}, {})", a, b, c),
            Self(None) => write!(f, "timed out"),
        }
    }
}

const TIMEOUT: Duration = Duration::from_millis(80);

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
    SearchResult::new(
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

    SearchResult::new(web, image, video)
}

// Don't wait for slow servers.
// https://talks.golang.org/2012/concurrency.slide#47
pub async fn search21(query: &SearchQuery) -> SearchResult {
    timeout(TIMEOUT, search20(query))
        .await
        .unwrap_or_else(|_| SearchResult::timeout())
}

// Reduce tail latency using replicated search servers.
// https://talks.golang.org/2012/concurrency.slide#50
pub async fn search30(query: &SearchQuery) -> SearchResult {
    timeout(TIMEOUT, async {
        tokio::join!(
            fastest(query, &SearchKind::Web),
            fastest(query, &SearchKind::Image),
            fastest(query, &SearchKind::Video),
        )
    })
    .await
    .map(|(web, image, video)| SearchResult::new(web, image, video))
    .unwrap_or_else(|_| SearchResult::timeout())
}

async fn fastest(query: &SearchQuery, kind: &SearchKind) -> String {
    let (server, result) = tokio::select!(
        r = fake_search(query, kind) => (1, r),
        r = fake_search(query, kind) => (2, r),
        r = fake_search(query, kind) => (3, r),
    );

    format!("Server {}: {}", server, result)
}
