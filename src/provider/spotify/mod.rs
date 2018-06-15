use provider;
use library::{Track, SharedLibrary, Album, Artist};
use failure::{Error, err_msg};
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

        let page = spotify.current_user_saved_albums(None, None)?;

        debug!("{:?}", page);

        Ok(provider::SyncResult {
            tracks: 0,
            albums: 0,
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

    fn search(&self, _query: String) -> Vec<provider::ProviderItem> {
        vec![]
    }

    fn resolve_track(&self, _uri: &str) -> Option<Track> {
        None
    }
}
