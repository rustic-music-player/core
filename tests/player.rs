extern crate rustic_core as core;
extern crate failure;
use core::PlayerBackend;

use std::sync::Arc;

fn create_test_track() -> core::Track {
    core::Track {
        id: None,
        title: String::from("Ukulele"),
        artist_id: None,
        artist: Some(core::Artist {
            id: None,
            name: String::from("Bensound"),
            uri: String::new(),
            image_url: None
        }),
        album_id: None,
        album: None,
        stream_url: String::from("file://local-provider/assets/bensound-ukulele.mp3"),
        provider: core::Provider::LocalMedia,
        uri: String::new(),
        image_url: None,
        duration: None
    }
}

#[test]
fn test_rodio_playback() {
    let track = create_test_track();
    let mut backend = core::RodioBackend::new().unwrap();
    let player = Arc::get_mut(&mut backend).unwrap();
    player.enqueue(&track);
    player.set_state(core::PlayerState::Play).unwrap();
}
