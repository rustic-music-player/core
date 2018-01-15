use provider::Provider;
use library::Track;

#[derive(Debug, Clone, Serialize)]
pub struct Playlist {
    pub id: Option<usize>,
    pub title: String,
    pub tracks: Vec<Track>,
    pub provider: Provider
}