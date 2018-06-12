use failure::Error;

mod explorer;
mod sync_error;
mod item;
mod folder;

// TODO: move provider into own packages
mod pocketcasts;
mod soundcloud;

pub use self::item::ProviderItem;
pub use self::folder::ProviderFolder;
pub use self::sync_error::SyncError;
pub use self::explorer::Explorer;

pub use self::pocketcasts::PocketcastsProvider;
pub use self::soundcloud::SoundcloudProvider;

use std::sync::{Arc, RwLock};
use library::{SharedLibrary, Track};

pub type SharedProviders = Vec<Arc<RwLock<Box<ProviderInstance + Send + Sync>>>>;

pub struct SyncResult {
    pub tracks: usize,
    pub albums: usize,
    pub artists: usize,
    pub playlists: usize
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Provider {
    Pocketcasts,
    Soundcloud,
    GooglePlayMusic,
    Spotify,
    LocalMedia
}

pub trait ProviderInstance {
    fn setup(&mut self) -> Result<(), Error>;
    fn title(&self) -> &'static str;
    fn uri_scheme(&self) -> &'static str;
    fn sync(&mut self, library: SharedLibrary) -> Result<SyncResult, Error>;
    fn root(&self) -> ProviderFolder;
    fn navigate(&self, path: Vec<String>) -> Result<ProviderFolder, Error>;
    fn search(&self, query: String) -> Vec<ProviderItem>;
    fn resolve_track(&self, uri: &str) -> Option<Track>;
}

#[derive(Debug, Fail)]
pub enum NavigationError {
    #[fail(display = "Path not found")]
    PathNotFound,
    #[fail(display = "can't fetch")]
    FetchError
}