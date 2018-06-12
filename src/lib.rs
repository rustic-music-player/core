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

pub mod cache;
pub mod bus;
pub mod library;
pub mod player;
pub mod provider;
pub mod sync;
pub mod store;
pub mod error;

pub use provider::{Explorer, Provider};
pub use library::{SharedLibrary, Library, Track, Artist, Album, Playlist};
pub use player::SharedPlayer;
pub use store::LibraryStore;
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
    pub fn new(providers: provider::SharedProviders) -> Result<Arc<Rustic>, failure::Error> {
        let bus = bus::MessageBus::new();
        Ok(Arc::new(Rustic {
            player: player::Player::new(Arc::clone(&bus))?,
            library: library::Library::new(),
            providers,
            bus,
            cache: Arc::new(cache::Cache::new())
        }))
    }
}