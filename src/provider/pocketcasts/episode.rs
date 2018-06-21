use library::Track;
use provider::Provider;
use pocketcasts::Episode;

impl From<Episode> for Track {
    fn from(episode: Episode) -> Track {
        Track {
            id: None,
            title: episode.title,
            artist_id: None,
            artist: None,
            album_id: None,
            album: None,
            stream_url: episode.url,
            provider: Provider::Pocketcasts,
            uri: format!("pocketcasts://episode/{}", episode.uuid),
            image_url: None,
            duration: episode.duration
        }
    }
}