use provider::Provider;
use std::cmp::Ordering;

#[derive(Clone, Debug, Serialize)]
pub struct Track {
    pub id: Option<usize>,
    pub title: String,
    pub artist_id: Option<usize>,
    pub album_id: Option<usize>,
    pub stream_url: String,
    pub provider: Provider,
    pub uri: String,
    pub coverart: Option<String>,
    pub duration: Option<u64>
}

impl PartialEq for Track {
    fn eq(&self, other: &Track) -> bool {
        self.uri == other.uri
    }
}

impl Eq for Track {}

impl PartialOrd for Track {
    fn partial_cmp(&self, other: &Track) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Track {
    fn cmp(&self, other: &Track) -> Ordering {
        self.title.cmp(&other.title)
    }
}
