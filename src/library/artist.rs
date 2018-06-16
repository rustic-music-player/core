use Rustic;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize)]
pub struct Artist {
    pub id: Option<usize>,
    pub name: String,
    pub uri: String,
    pub image_url: Option<String>
}

impl Artist {
    pub fn image(&self, app: &Arc<Rustic>) -> Option<String> {
        self.image_url
            .clone()
            .and_then(|uri| app.cache.fetch_coverart(uri).ok())
    }
}

impl PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        self.uri == other.uri
    }
}

impl Eq for Artist {}
