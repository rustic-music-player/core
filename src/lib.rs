#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate rayon;
extern crate gstreamer;
extern crate glib;
extern crate libc;
extern crate url;
extern crate soundcloud;
extern crate pocketcasts;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate md5;
extern crate reqwest;
extern crate image;
extern crate rspotify;
extern crate rustic_local_provider;

pub mod cache;
pub mod bus;
pub mod library;
pub mod player;
pub mod provider;
pub mod sync;
pub mod error;

pub use provider::{Explorer, Provider};
pub use library::{SharedLibrary, Library, Track, Artist, Album, Playlist, SearchResults};
pub use player::SharedPlayer;
pub use error::RusticError;

use std::sync::Arc;

pub struct Rustic {
    pub bus: bus::SharedBus,
    pub player: player::SharedPlayer,
    pub library: library::SharedLibrary,
    pub providers: provider::SharedProviders,
    pub cache: cache::SharedCache
}

impl Rustic {
    pub fn new(library: Box<Library>, providers: provider::SharedProviders) -> Result<Arc<Rustic>, failure::Error> {
        let library = Arc::new(library);
        let bus = bus::MessageBus::new();
        Ok(Arc::new(Rustic {
            player: player::Player::new(Arc::clone(&bus))?,
            library,
            providers,
            bus,
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