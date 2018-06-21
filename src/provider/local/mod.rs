use provider::*;
use library::{self, SharedLibrary};
use failure::Error;
use rustic_local_provider as local;

#[derive(Clone, Deserialize, Debug)]
pub struct LocalProvider {
    path: String
}

impl ProviderInstance for LocalProvider {

    fn title(&self) -> &'static str {
        "Local"
    }

    fn uri_scheme(&self) -> &'static str { "file" }

    fn setup(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn sync(&mut self, library: SharedLibrary) -> Result<SyncResult, Error> {
        let scanner = local::Scanner::new(self.path.clone());
        let tracks = scanner.scan()?;
        let mut tracks = tracks
            .into_iter()
            .map(library::Track::from)
            .collect();
        library.add_tracks(&mut tracks);
        Ok(SyncResult {
            tracks: tracks.len(),
            albums: 0,
            artists: 0,
            playlists: 0
        })
    }

    fn root(&self) -> ProviderFolder {
        ProviderFolder {
            folders: vec![],
            items: vec![],
        }
    }

    fn navigate(&self, path: Vec<String>) -> Result<ProviderFolder, Error> {
        Ok(self.root())
    }

    fn search(&self, query: String) -> Result<Vec<ProviderItem>, Error> {
        Ok(vec![])
    }

    fn resolve_track(&self, _uri: &str) -> Result<Option<library::Track>, Error> {
        Ok(None)
    }
}

impl From<local::Track> for library::Track {
    fn from(track: local::Track) -> Self {
        library::Track {
            id: None,
            title: track.title,
            album_id: None,
            album: track.album.map(|name| library::Album {
                id: None,
                title: name,
                artist_id: None,
                artist: None,
                provider: Provider::LocalMedia,
                image_url: None,
                uri: String::new(),
            }),
            artist_id: None,
            artist: track.artist.map(|name| library::Artist {
                id: None,
                name,
                uri: String::new(),
                image_url: None
            }),
            image_url: None,
            stream_url: format!("file://{}", track.path),
            provider: Provider::LocalMedia,
            uri: format!("file://{}", track.path),
            duration: None
        }
    }
}