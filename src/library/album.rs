use provider::Provider;
use std::sync::Arc;
use Rustic;

#[derive(Clone, Debug, Serialize)]
pub struct Album {
    pub id: Option<usize>,
    pub title: String,
    pub artist_id: Option<usize>,
    pub provider: Provider,
    pub image_url: Option<String>,
    pub uri: String
}

impl Album {
    pub fn coverart(&self, app: &Arc<Rustic>) -> Option<String> {
        self.image_url
            .clone()
            .and_then(|uri| app.cache.fetch_coverart(uri).ok())
    }
}