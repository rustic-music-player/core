#[derive(Clone, Debug, Serialize)]
pub struct Artist {
    pub id: Option<usize>,
    pub name: String,
    pub uri: String
}

impl PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        self.uri == other.uri
    }
}

impl Eq for Artist {}
