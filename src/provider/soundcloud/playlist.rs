use soundcloud;
use provider;
use library::{Playlist, Track};
use super::track::SoundcloudTrack;

#[derive(Debug, Clone)]
pub struct SoundcloudPlaylist {
    pub id: u64,
    pub title: String,
    pub tracks: Vec<SoundcloudTrack>
}

impl SoundcloudPlaylist {
    pub fn from(playlist: soundcloud::Playlist, client_id: String) -> SoundcloudPlaylist {
        SoundcloudPlaylist {
            id: playlist.id,
            title: playlist.title,
            tracks: playlist
                .tracks
                .iter()
                .cloned()
                .map(SoundcloudTrack::from)
                .filter(|track| track.url.is_some())
                .map(|track| SoundcloudTrack {
                    id: track.id,
                    title: track.title,
                    url: track.url.map(|url| format!("{}?client_id={}", url, client_id)),
                    coverart: track.coverart,
                    duration: track.duration
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
            tracks: playlist.tracks
                .iter()
                .cloned()
                .map(SoundcloudTrack::from)
                .map(Track::from)
                .collect(),
            provider: provider::Provider::Soundcloud
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
                .map(SoundcloudTrack::from)
                .filter(|track| track.url.is_some())
                .collect()
        }
    }
}