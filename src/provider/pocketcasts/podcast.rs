use library::{Album, Artist};
use provider::Provider;
use pocketcasts::Podcast;

impl From<Podcast> for Album {
    fn from(podcast: Podcast) -> Album {
        Album {
            id: None,
            title: podcast.title,
            artist_id: None,
            artist: Some(Artist {
                id: None,
                uri: format!("pocketcasts://interpret/{}", podcast.author),
                name: podcast.author,
                image_url: None
            }),
            provider: Provider::Pocketcasts,
            image_url: podcast.thumbnail_url,
            uri: format!("pocketcasts://podcast/{}", podcast.uuid)
        }
    }
}

impl From<Podcast> for Artist {
    fn from(podcast: Podcast) -> Artist {
        Artist {
            id: None,
            uri: format!("pocketcasts://interpret/{}", podcast.author),
            name: podcast.author,
            image_url: None
        }
    }
}
