use soundcloud;
use provider;
use library::{Track, Artist};

impl From<soundcloud::Track> for Track {
    fn from(track: soundcloud::Track) -> Track {
        Track {
            id: None,
            title: track.title,
            artist: Some(Artist {
                id: None,
                name: track.user.username,
                image_url: Some(track.user.avatar_url),
                uri: format!("soundcloud://user/{}", track.user.id)
            }),
            artist_id: None,
            album: None,
            album_id: None,
            stream_url: track.stream_url.unwrap(),
            provider: provider::Provider::Soundcloud,
            uri: format!("soundcloud://track/{}", track.id),
            image_url: track.artwork_url,
            duration: Some(track.duration)
        }
    }
}

impl From<soundcloud::Track> for provider::ProviderItem {
    fn from(track: soundcloud::Track) -> provider::ProviderItem {
        provider::ProviderItem::from(Track::from(track))
    }
}