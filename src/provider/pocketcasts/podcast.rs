use library::{Album, Artist};
use provider::Provider;
use pocketcasts::Podcast;

impl From<Podcast> for Album {
    fn from(podcast: Podcast) -> Album {
        Album {
            id: None,
            title: podcast.title,
            artist_id: None,
            provider: Provider::Pocketcasts,
            coverart: podcast.thumbnail_url
        }
    }
}

impl From<Podcast> for Artist {
    fn from(podcast: Podcast) -> Artist {
        Artist {
            id: None,
            name: podcast.author
        }
    }
}
