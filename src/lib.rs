#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
#[macro_use]
extern crate lazy_static;
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
extern crate reqwest;

pub mod bus;
pub mod library;
pub mod player;
pub mod provider;
pub mod logger;
pub mod sync;

pub use provider::{Explorer, Provider};
pub use library::{SharedLibrary, Track, Artist, Album, Playlist};
pub use player::SharedPlayer;

use std::sync::Arc;

pub struct Rustic {
    pub bus: bus::SharedBus,
    pub player: player::SharedPlayer,
    pub library: library::SharedLibrary,
    pub providers: provider::SharedProviders
}

impl Rustic {
    pub fn new(providers: provider::SharedProviders) -> Arc<Rustic> {
        let bus = bus::MessageBus::new();
        Arc::new(Rustic {
            player: player::Player::new(Arc::clone(&bus)),
            library: library::Library::new(),
            providers,
            bus
        })
    }
}