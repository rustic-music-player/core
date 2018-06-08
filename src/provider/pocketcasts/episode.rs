use library::Track;
use provider::Provider;
use pocketcasts::Episode;

impl From<Episode> for Track {
    fn from(episode: Episode) -> Track {
        Track {
            id: None,
            title: episode.title,
            artist_id: None,
            album_id: None,
            stream_url: episode.url,
            provider: Provider::Pocketcasts,
            uri: format!("pocketcasts://{}", episode.uuid),
            coverart: None,
            duration: episode.duration,
        }
    }
}