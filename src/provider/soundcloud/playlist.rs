use soundcloud;
use provider;
use library::{Playlist, Track};

#[derive(Debug, Clone)]
pub struct SoundcloudPlaylist {
    pub id: u64,
    pub title: String,
    pub tracks: Vec<Track>
}

impl SoundcloudPlaylist {
    pub fn from(playlist: soundcloud::Playlist, client_id: &str) -> SoundcloudPlaylist {
        SoundcloudPlaylist {
            id: playlist.id,
            title: playlist.title,
            tracks: playlist
                .tracks
                .iter()
                .cloned()
                .filter(|track| track.stream_url.is_some())
                .map(Track::from)
                .map(|track| Track {
                    stream_url: format!("{}?client_id={}", track.stream_url, client_id),
                    ..track
                })
                .collect()
        }
    }
}

impl From<SoundcloudPlaylist> for Playlist {
    fn from(playlist: SoundcloudPlaylist) -> Playlist {
        Playlist {
            id: None,
            title: playlist.title,
            tracks: playlist.tracks,
            provider: provider::Provider::Soundcloud,
            uri: format!("soundcloud://playlist/{}", playlist.id)
        }
    }
}

impl From<soundcloud::Playlist> for SoundcloudPlaylist {
    fn from(playlist: soundcloud::Playlist) -> SoundcloudPlaylist {
        SoundcloudPlaylist {
            id: playlist.id,
            title: playlist.title,
            tracks: playlist
                .tracks
                .iter()
                .cloned()
                .filter(|track| track.stream_url.is_some())
                .map(Track::from)
                .collect()
        }
    }
}