mod episode;
mod podcast;

use provider;
use library::{Track, SharedLibrary, Album, Artist};
use rayon::prelude::*;
use pocketcasts::{Podcast, Episode, PocketcastClient};
use failure::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct PocketcastsProvider {
    email: String,
    password: String,
    #[serde(skip)]
    client: Option<PocketcastClient>
}

impl provider::ProviderInstance for PocketcastsProvider {
    fn title(&self) -> &'static str {
        "Pocketcasts"
    }

    fn setup(&mut self) -> Result<(), Error> {
        let mut client = PocketcastClient::new(self.email.clone(), self.password.clone());
        client.login()?;
        self.client = Some(client);

        Ok(())
    }

    fn uri_scheme(&self) -> &'static str { "pocketcasts" }

    fn sync(&mut self, library: SharedLibrary) -> Result<provider::SyncResult, Error> {
        let client = self.client.clone().unwrap();
        let podcasts = client.get_subscriptions()?;
        let albums = podcasts.len();
        let mut episodes: Vec<Track> = podcasts
            .par_iter()
            .cloned()
            .map(|podcast| {
                let episodes = client.get_episodes(&podcast)?;
                Ok((podcast, episodes))
            })
            .filter(|result: &Result<(Podcast, Vec<Episode>), Error>| {
                debug!("{:?}", result);
                result.is_ok()
            })
            .map(|result| result.unwrap())
            .map(|(podcast, episodes)| {
                let mut artist = Artist::from(podcast.clone());
                let mut album = Album::from(podcast);
                library.sync_artist(&mut artist);
                album.artist_id = artist.id;
                library.sync_album(&mut album);
                let tracks: Vec<Track> = episodes
                    .iter()
                    .cloned()
                    .map(Track::from)
                    .map(|mut track| {
                        track.album_id = album.id;
                        track.artist_id = artist.id;
                        track.image_url = album.image_url.clone();
                        track
                    })
                    .collect();
                tracks
            })
            .reduce(|| vec![], |mut a, b| {
                a.extend(b);
                a
            });
        let tracks = episodes.len();
        library.sync_tracks(&mut episodes);
        Ok(provider::SyncResult {
            tracks,
            albums,
            artists: albums,
            playlists: 0
        })
    }

    fn root(&self) -> provider::ProviderFolder {
        provider::ProviderFolder {
            folders: vec![
                "Subscriptions".to_owned(),
                "Top Charts".to_owned(),
                "Featured".to_owned(),
                "Trending".to_owned()
            ],
            items: vec![]
        }
    }

    fn navigate(&self, path: Vec<String>) -> Result<provider::ProviderFolder, Error> {
        let client = self.client.clone().unwrap();
        let podcasts = match path[0].as_str() {
            "Subscriptions" => Ok(self.client.clone().unwrap().get_subscriptions()),
            "Top Charts" => Ok(PocketcastClient::get_top_charts()),
            "Featured" => Ok(PocketcastClient::get_featured()),
            "Trending" => Ok(PocketcastClient::get_trending()),
            _ => Err(Error::from(provider::NavigationError::PathNotFound))
        }?;
        match path.len() {
            1 => podcasts.map(provider::ProviderFolder::from),
            2 => podcasts.and_then(|podcasts| {
                podcasts
                    .iter()
                    .find(|podcast| podcast.title == path[1])
                    .ok_or(provider::NavigationError::PathNotFound)
                    .map_err(Error::from)
                    .and_then(|podcast| client.get_episodes(&podcast)
                        .map_err(|_err| Error::from(provider::NavigationError::FetchError)))
                    .map(|episodes| {
                        episodes
                            .iter()
                            .cloned()
                            .map(Track::from)
                            .map(provider::ProviderItem::from)
                            .collect()
                    })
                    // .ok_or(Error::from(provider::NavigationError::FetchError))
                    .map(|items| {
                        provider::ProviderFolder {
                            folders: vec![],
                            items
                        }
                    })
            }),
            _ => Err(Error::from(provider::NavigationError::PathNotFound))
        }
    }

    fn search(&self, query: String) -> Result<Vec<provider::ProviderItem>, Error> {
        let client = self.client.clone().unwrap();
        let podcasts = client.search_podcasts(query)?;
        let podcasts = podcasts
            .into_iter()
            .map(Album::from)
            .map(provider::ProviderItem::from)
            .collect();
        Ok(podcasts)
    }

    fn resolve_track(&self, _uri: &str) -> Result<Option<Track>, Error> {
        Ok(None)
    }
}

impl From<Vec<Podcast>> for provider::ProviderFolder {
    fn from(podcasts: Vec<Podcast>) -> provider::ProviderFolder {
        provider::ProviderFolder {
            folders: podcasts
                .iter()
                .cloned()
                .map(|podcast| podcast.title)
                .collect(),
            items: vec![]
        }
    }
}