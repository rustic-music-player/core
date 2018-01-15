use provider::Provider;

#[derive(Clone, Debug, Serialize)]
pub struct Album {
    pub id: Option<usize>,
    pub title: String,
    pub artist_id: Option<usize>,
    pub provider: Provider,
    pub coverart: Option<String>
}