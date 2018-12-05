use failure::Error;

mod explorer;
mod sync_error;
mod item;
mod folder;

pub use self::item::{ProviderItem, ProviderItemType};
pub use self::folder::ProviderFolder;
pub use self::sync_error::SyncError;
pub use self::explorer::Explorer;

use std::sync::{Arc, RwLock};
use std::fmt::Debug;
use library::{SharedLibrary, Track};

pub type SharedProviders = Vec<Arc<RwLock<Box<ProviderInstance + Send + Sync>>>>;

pub struct SyncResult {
    pub tracks: usize,
    pub albums: usize,
    pub artists: usize,
    pub playlists: usize
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Provider {
    Pocketcasts,
    Soundcloud,
    GooglePlayMusic,
    Spotify,
    #[serde(rename = "local")]
    LocalMedia
}

pub trait ProviderInstance: Debug {
    fn setup(&mut self) -> Result<(), Error>;
    fn title(&self) -> &'static str;
    fn uri_scheme(&self) -> &'static str;
    fn sync(&mut self, library: SharedLibrary) -> Result<SyncResult, Error>;
    fn root(&self) -> ProviderFolder;
    fn navigate(&self, path: Vec<String>) -> Result<ProviderFolder, Error>;
    fn search(&self, query: String) -> Result<Vec<ProviderItem>, Error>;
    fn resolve_track(&self, uri: &str) -> Result<Option<Track>, Error>;
}

#[derive(Debug, Fail)]
pub enum NavigationError {
    #[fail(display = "Path not found")]
    PathNotFound,
    #[fail(display = "can't fetch")]
    FetchError
}