use library::{Artist, Album, Track, Playlist};
use provider::SharedProviders;
use std::sync::{RwLock, Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use url::Url;
use failure::Error;

#[derive(Debug, Serialize)]
pub struct InMemoryLibrary {
    #[serde(skip_serializing)]
    album_id: AtomicUsize,
    #[serde(skip_serializing)]
    artist_id: AtomicUsize,
    #[serde(skip_serializing)]
    track_id: AtomicUsize,
    #[serde(skip_serializing)]
    playlist_id: AtomicUsize,
    pub albums: RwLock<Vec<Album>>,
    pub artists: RwLock<Vec<Artist>>,
    pub tracks: RwLock<Vec<Track>>,
    pub playlists: RwLock<Vec<Playlist>>
}

pub type SharedLibrary = Arc<Box<Library + Send + Sync>>;

impl InMemoryLibrary {
    pub fn new() -> SharedLibrary {
        Arc::new(Box::new(InMemoryLibrary {
            album_id: AtomicUsize::new(1),
            artist_id: AtomicUsize::new(1),
            track_id: AtomicUsize::new(1),
            playlist_id: AtomicUsize::new(1),
            albums: RwLock::new(vec![]),
            artists: RwLock::new(vec![]),
            tracks: RwLock::new(vec![]),
            playlists: RwLock::new(vec![])
        }))
    }


    pub fn resolve_track(&self, providers: &SharedProviders, uri: &String) -> Result<Option<Track>, Error> {
        let track = self.tracks
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|track| &track.uri == uri);

        match track {
            Some(track) => Ok(Some(track)),
            None => {
                let url = Url::parse(uri)?;
                let provider = providers
                    .iter()
                    .find(|provider| provider.read().unwrap().uri_scheme() == url.scheme());
                let track = match provider {
                    Some(provider) => provider.read().unwrap().resolve_track(uri)?,
                    _ => None
                };
                Ok(track)
            }
        }
    }
}

pub struct SearchResults {
    tracks: Vec<Track>,
    albums: Vec<Album>,
    artists: Vec<Artist>,
    playlists: Vec<Playlist>
}

pub trait Library {
    fn get_track(&self, id: &usize) -> Result<Option<Track>, Error>;
    fn get_tracks(&self) -> Result<Vec<Track>, Error>;
    
    fn get_album(&self, id: &usize) -> Result<Option<Album>, Error>;
    fn get_albums(&self) -> Result<Vec<Album>, Error>;
    
    fn get_artist(&self, id: &usize) -> Result<Option<Artist>, Error>;
    fn get_artists(&self) -> Result<Vec<Artist>, Error>;
    
    fn get_playlist(&self, id: &usize) -> Result<Option<Playlist>, Error>;
    fn get_playlists(&self) -> Result<Vec<Playlist>, Error>;

    fn add_track(&self, track: &mut Track) -> Result<(), Error>;
    fn add_album(&self, album: &mut Album) -> Result<(), Error>;
    fn add_artist(&self, artist: &mut Artist) -> Result<(), Error>;
    fn add_playlist(&self, playlist: &mut Playlist) -> Result<(), Error>;

    fn add_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error>;
    fn add_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error>;
    fn add_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error>;
    fn add_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error>;

    fn sync_track(&self, track: &mut Track) -> Result<(), Error>;
    fn sync_album(&self, album: &mut Album) -> Result<(), Error>;
    fn sync_artist(&self, artist: &mut Artist) -> Result<(), Error>;
    fn sync_playlist(&self, playlist: &mut Playlist) -> Result<(), Error>;

    fn sync_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error>;
    fn sync_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error>;
    fn sync_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error>;
    fn sync_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error>;

    fn search(&self, query: String) -> Result<SearchResults, Error>;
}

impl Library for InMemoryLibrary {

    fn get_track(&self, id: &usize) -> Result<Option<Track>, Error> {
        let track = self.tracks
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|track| track.id == Some(*id));
        Ok(track)
    }

    fn get_tracks(&self) -> Result<Vec<Track>, Error> {
        let tracks = self.tracks
            .read()
            .unwrap()
            .clone();
        Ok(tracks)
    }

    fn get_album(&self, id: &usize) -> Result<Option<Album>, Error> {
        let album = self.albums
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|album| album.id == Some(*id));
        Ok(album)
    }

    fn get_albums(&self) -> Result<Vec<Album>, Error> {
        let albums = self.albums
            .read()
            .unwrap()
            .clone();
        Ok(albums)
    }

    fn get_artist(&self, id: &usize) -> Result<Option<Artist>, Error> {
        let artist = self.artists
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|artist| artist.id == Some(*id));
        Ok(artist)
    }

    fn get_artists(&self) -> Result<Vec<Artist>, Error> {
        let artists = self.artists
            .read()
            .unwrap()
            .clone();
        Ok(artists)
    }

    fn get_playlist(&self, id: &usize) -> Result<Option<Playlist>, Error> {
        let playlist = self.playlists
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|playlist| playlist.id == Some(*id));
        Ok(playlist)
    }

    fn get_playlists(&self) -> Result<Vec<Playlist>, Error> {
        let playlists = self.playlists
            .read()
            .unwrap()
            .clone();
        Ok(playlists)
    }

    fn add_track(&self, track: &mut Track) -> Result<(), Error> {
        track.id = Some(self.track_id.fetch_add(1, Ordering::Relaxed));
        self.tracks.write().unwrap().push(track.clone());
        Ok(())
    }

    fn add_album(&self, album: &mut Album) -> Result<(), Error> {
        album.id = Some(self.album_id.fetch_add(1, Ordering::Relaxed));
        self.albums.write().unwrap().push(album.clone());
        Ok(())
    }

    fn add_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        artist.id = Some(self.artist_id.fetch_add(1, Ordering::Relaxed));
        self.artists.write().unwrap().push(artist.clone());
        Ok(())
    }

    fn add_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        playlist.id = Some(self.playlist_id.fetch_add(1, Ordering::Relaxed));
        self.playlists.write().unwrap().push(playlist.clone());
        Ok(())
    }

    fn add_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error> {
        let tracks = tracks
            .iter()
            .cloned()
            .map(|mut track| {
                track.id = Some(self.track_id.fetch_add(1, Ordering::Relaxed));
                track
            });
        self.tracks.write().unwrap().extend(tracks);
        Ok(())
    }

    fn add_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error> {
        let albums = albums
            .iter()
            .cloned()
            .map(|mut album| {
                album.id = Some(self.album_id.fetch_add(1, Ordering::Relaxed));
                album
            });
        self.albums.write().unwrap().extend(albums);
        Ok(())
    }

    fn add_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error> {
        let artists = artists
            .iter()
            .cloned()
            .map(|mut artist| {
                artist.id = Some(self.artist_id.fetch_add(1, Ordering::Relaxed));
                artist
            });
        self.artists.write().unwrap().extend(artists);
        Ok(())
    }

    fn add_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        let playlists = playlists
            .iter()
            .cloned()
            .map(|mut playlist| {
                playlist.id = Some(self.playlist_id.fetch_add(1, Ordering::Relaxed));
                playlist
            });
        self.playlists.write().unwrap().extend(playlists);
        Ok(())
    }

    fn sync_track(&self, track: &mut Track) -> Result<(), Error> {
        let has_track = {
            let tracks = self.tracks.read().unwrap();
            tracks
                .iter()
                .find(|a| a.uri == track.uri)
                .map(|a| a.id)
        };

        let id: usize = has_track
            .and_then(|id| id)
            .unwrap_or_else(|| self.track_id.fetch_add(1, Ordering::Relaxed));
        track.id = Some(id);

        if has_track.is_none() {
            self.tracks
                .write()
                .unwrap()
                .push(track.clone());
        }
        Ok(())
    }

    fn sync_album(&self, album: &mut Album) -> Result<(), Error> {
        let has_album = {
            let albums = self.albums.read().unwrap();
            albums
                .iter()
                .find(|a| a.uri == album.uri)
                .map(|a| a.id)
        };

        let id: usize = has_album
            .and_then(|id| id)
            .unwrap_or_else(|| self.album_id.fetch_add(1, Ordering::Relaxed));
        album.id = Some(id);

        if has_album.is_none() {
            self.albums
                .write()
                .unwrap()
                .push(album.clone());
        }
        Ok(())
    }

    fn sync_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        let has_artist = {
            let artists = self.artists.read().unwrap();
            artists
                .iter()
                .find(|a| a.uri == artist.uri)
                .map(|a| a.id)
        };

        let id: usize = has_artist
            .and_then(|id| id)
            .unwrap_or_else(|| self.artist_id.fetch_add(1, Ordering::Relaxed));
        artist.id = Some(id);

        if has_artist.is_none() {
            self.artists
                .write()
                .unwrap()
                .push(artist.clone());
        }
        Ok(())
    }

    fn sync_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        let has_playlist = {
            let playlists = self.playlists.read().unwrap();
            playlists
                .iter()
                .find(|a| a.uri == playlist.uri)
                .map(|a| a.id)
        };

        let id: usize = has_playlist
            .and_then(|id| id)
            .unwrap_or_else(|| self.playlist_id.fetch_add(1, Ordering::Relaxed));
        playlist.id = Some(id);

        if has_playlist.is_none() {
            self.playlists
                .write()
                .unwrap()
                .push(playlist.clone());
        }
        Ok(())
    }

    fn sync_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error> {
        tracks
            .into_iter()
            .filter(|track| {
                let tracks = self.tracks.read().unwrap();
                tracks
                    .iter()
                    .find(|t| t.uri == track.uri)
                    .map(|_t| false)
                    .unwrap_or(true)
            })
            .map(|mut track| self.add_track(&mut track))
            .collect()
    }

    fn sync_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error> {
        artists
            .into_iter()
            .filter(|artist| {
                let artists = self.artists.read().unwrap();
                artists
                    .iter()
                    .find(|t| t.uri == artist.uri)
                    .map(|_t| false)
                    .unwrap_or(true)
            })
            .map(|mut artist| self.add_artist(&mut artist))
            .collect()
    }

    fn sync_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error> {
        albums
            .into_iter()
            .filter(|album| {
                let albums = self.albums.read().unwrap();
                albums
                    .iter()
                    .find(|t| t.uri == album.uri)
                    .map(|_t| false)
                    .unwrap_or(true)
            })
            .map(|mut album| self.add_album(&mut album))
            .collect()
    }

    fn sync_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        playlists
            .into_iter()
            .filter(|playlist| {
                let playlists = self.playlists.read().unwrap();
                playlists
                    .iter()
                    .find(|p| p.uri == playlist.uri)
                    .map(|_p| false)
                    .unwrap_or(true)
            })
            .map(|mut p| self.add_playlist(&mut p))
            .collect()
    }

    fn search(&self, query: String) -> Result<SearchResults, Error> {
        let tracks = self.tracks
            .read()
            .unwrap()
            .iter()
            .cloned()
            .filter(|track| track.title.contains(query.as_str()))
            .collect();

        Ok(SearchResults {
            tracks,
            albums: vec![],
            artists: vec![],
            playlists: vec![]
        })
    }
}