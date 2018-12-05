#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate url;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate md5;
extern crate reqwest;
extern crate image;
extern crate crossbeam_channel as channel;

pub mod cache;
pub mod library;
pub mod provider;
pub mod sync;
pub mod player;

pub use provider::{Explorer, Provider};
pub use library::{SharedLibrary, Library, Track, Artist, Album, Playlist, SearchResults};
pub use player::{PlayerBackend, PlayerState, PlayerEvent};

use std::sync::Arc;

pub struct Rustic {
    pub player: Arc<Box<PlayerBackend>>,
    pub library: library::SharedLibrary,
    pub providers: provider::SharedProviders,
    pub cache: cache::SharedCache
}

impl Rustic {
    pub fn new(library: Box<Library>, providers: provider::SharedProviders, player: Arc<Box<PlayerBackend>>) -> Result<Arc<Rustic>, failure::Error> {
        let library = Arc::new(library);
        Ok(Arc::new(Rustic {
            player,
            library,
            providers,
            cache: Arc::new(cache::Cache::new())
        }))
    }

    pub fn resolve_track(&self, uri: &String) -> Result<Option<Track>, failure::Error> {
        let track = self
            .library
            .get_tracks()?
            .into_iter()
            .find(|track| &track.uri == uri);

        match track {
            Some(track) => Ok(Some(track)),
            None => {
                let url = url::Url::parse(uri)?;
                let provider = self
                    .providers
                    .iter()
                    .find(|provider| provider.read().unwrap().uri_scheme() == url.scheme());
                let track = match provider {
                    Some(provider) => provider.read().unwrap().resolve_track(uri)?,
                    _ => None
                };
                Ok(track)
            }
        }
    }

}