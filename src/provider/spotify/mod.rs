use provider;
use library::{Track, SharedLibrary, Album, Artist};
use failure::{Error, err_msg};
use rspotify::spotify::model::{
    image::Image,
    album::{FullAlbum, SimplifiedAlbum},
    artist::{FullArtist, SimplifiedArtist},
    track::{FullTrack, SimplifiedTrack}
};
use rspotify::spotify::client::Spotify;
use rspotify::spotify::util::get_token;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};

#[derive(Clone, Deserialize, Debug)]
pub struct SpotifyProvider {
    client_id: String,
    client_secret: String,
    #[serde(skip)]
    client: Option<Spotify>
}

impl provider::ProviderInstance for SpotifyProvider {

    fn title(&self) -> &'static str {
        "Spotify"
    }

    fn uri_scheme(&self) -> &'static str { "spotify" }

    fn setup(&mut self) -> Result<(), Error> {
        let mut oauth = SpotifyOAuth::default()
            .client_id(&self.client_id)
            .client_secret(&self.client_secret)
            .scope(&[
                "user-library-read",
                "playlist-read-private",
                "user-top-read",
                "user-read-recently-played",
                "playlist-read-collaborative"
            ].join(" "))
            .redirect_uri("http://localhost:8888/callback")
            .build();

        let spotify = get_token(&mut oauth)
            .map(|token_info| {
                let client_credential = SpotifyClientCredentials::default()
                    .token_info(token_info)
                    .build();
                Spotify::default()
                    .client_credentials_manager(client_credential)
                    .build()
            })
            .ok_or(err_msg("Spotify auth failed"))?;

        self.client = Some(spotify);

        Ok(())
    }

    fn sync(&mut self, library: SharedLibrary) -> Result<provider::SyncResult, Error> {
        let spotify = self.client.clone().unwrap();

        let albums = spotify.current_user_saved_albums(None, None)?.items;

        debug!("{:?}", albums);

        let albums_len = albums.len();

        let mut tracks = albums
            .into_iter()
            .map(|album| album.album)
            .map(|album| {
                let mut album_entity = Album::from(album.clone());
                library.sync_album(&mut album_entity);
                album.tracks.items
                    .into_iter()
                    .map(Track::from)
                    .map(|mut track| {
                        track.album_id = album_entity.id;
                        track
                    })
                    .collect()
            })
            .fold(vec![], |mut a, b: Vec<Track>| {
                a.extend(b);
                a
            });

        library.sync_tracks(&mut tracks);

        Ok(provider::SyncResult {
            tracks: tracks.len(),
            albums: albums_len,
            artists: 0,
            playlists: 0
        })
    }

    fn root(&self) -> provider::ProviderFolder {
        provider::ProviderFolder {
            folders: vec![],
            items: vec![],
        }
    }

    fn navigate(&self, path: Vec<String>) -> Result<provider::ProviderFolder, Error> {
        Ok(self.root())
    }

    fn search(&self, query: String) -> Result<Vec<provider::ProviderItem>, Error> {
        trace!("search {}", query);
        let spotify = self.client.clone().unwrap();

        let albums = spotify.search_album(&query, None, None, None)?;
        let artists = spotify.search_artist(&query, None, None, None)?;
        let tracks = spotify.search_track(&query, None, None, None)?;
        // let albums = spotify.search_album(&query, None, None, None)?;

        let albums = albums.albums.items
            .into_iter()
            .map(Album::from)
            .map(provider::ProviderItem::from);
        let artists = artists.artists.items
            .into_iter()
            .map(Artist::from)
            .map(provider::ProviderItem::from);
        let tracks = tracks.tracks.items
            .into_iter()
            .map(Track::from)
            .map(provider::ProviderItem::from);

        Ok(albums.chain(artists).chain(tracks).collect())
    }

    fn resolve_track(&self, _uri: &str) -> Result<Option<Track>, Error> {
        Ok(None)
    }
}

fn convert_images(images: &Vec<Image>) -> Option<String> {
    images.first().map(|image| image.url.clone())
}

impl From<FullAlbum> for Album {
    fn from(album: FullAlbum) -> Self {
        Album {
            id: None,
            title: album.name,
            artist_id: None,
            provider: provider::Provider::Spotify,
            image_url: convert_images(&album.images),
            uri: format!("spotify://album/{}", album.id)
        }
    }
}

impl From<SimplifiedAlbum> for Album {
    fn from(album: SimplifiedAlbum) -> Self {
        Album {
            id: None,
            title: album.name,
            artist_id: None,
            provider: provider::Provider::Spotify,
            image_url: convert_images(&album.images),
            uri: format!("spotify://album/{}", album.id)
        }
    }
}

impl From<FullArtist> for Artist {
    fn from(artist: FullArtist) -> Self {
        Artist {
            id: None,
            name: artist.name,
            image_url: convert_images(&artist.images),
            uri: format!("spotify://artist/{}", artist.id)
        }
    }
}

impl From<SimplifiedArtist> for Artist {
    fn from(artist: SimplifiedArtist) -> Self {
        Artist {
            id: None,
            name: artist.name,
            image_url: None,
            uri: format!("spotify://artist/{}", artist.id)
        }
    }
}

impl From<FullTrack> for Track {
    fn from(track: FullTrack) -> Self {
        Track {
            id: None,
            title: track.name,
            artist_id: None,
            album_id: None,
            stream_url: String::new(),
            provider: provider::Provider::Spotify,
            image_url: convert_images(&track.album.images),
            uri: format!("spotify://track/{}", track.id),
            duration: Some(track.duration_ms as u64)
        }
    }
}

impl From<SimplifiedTrack> for Track {
    fn from(track: SimplifiedTrack) -> Self {
        Track {
            id: None,
            title: track.name,
            artist_id: None,
            album_id: None,
            stream_url: String::new(),
            provider: provider::Provider::Spotify,
            image_url: None,
            uri: format!("spotify://track/{}", track.id),
            duration: Some(track.duration_ms as u64)
        }
    }
}