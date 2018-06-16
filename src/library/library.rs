use library::{Artist, Album, Track, Playlist};
use provider::SharedProviders;
use std::sync::{RwLock, Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use url::Url;
use failure::Error;

#[derive(Debug, Serialize)]
pub struct Library {
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

pub type SharedLibrary = Arc<Library>;

impl Library {
    pub fn new() -> SharedLibrary {
        Arc::new(Library {
            album_id: AtomicUsize::new(1),
            artist_id: AtomicUsize::new(1),
            track_id: AtomicUsize::new(1),
            playlist_id: AtomicUsize::new(1),
            albums: RwLock::new(vec![]),
            artists: RwLock::new(vec![]),
            tracks: RwLock::new(vec![]),
            playlists: RwLock::new(vec![])
        })
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

    pub fn get_track(&self, id: &usize) -> Option<Track> {
        self.tracks
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|track| track.id == Some(*id))
    }

    pub fn get_album(&self, id: &usize) -> Option<Album> {
        self.albums
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|album| album.id == Some(*id))
    }

    pub fn get_artist(&self, id: &usize) -> Option<Artist> {
        self.artists
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|artist| artist.id == Some(*id))
    }

    pub fn get_playlist(&self, id: &usize) -> Option<Playlist> {
        self.playlists
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|playlist| playlist.id == Some(*id))
    }

    pub fn add_tracks(&self, tracks: &mut Vec<Track>) {
        let tracks = tracks
            .iter()
            .cloned()
            .map(|mut track| {
                track.id = Some(self.track_id.fetch_add(1, Ordering::Relaxed));
                track
            });
        self.tracks.write().unwrap().extend(tracks);
    }

    pub fn add_albums(&self, albums: &mut Vec<Album>) {
        let albums = albums
            .iter()
            .cloned()
            .map(|mut album| {
                album.id = Some(self.album_id.fetch_add(1, Ordering::Relaxed));
                album
            });
        self.albums.write().unwrap().extend(albums);
    }

    pub fn add_playlists(&self, playlists: &mut Vec<Playlist>) {
        let playlists = playlists
            .iter()
            .cloned()
            .map(|mut playlist| {
                playlist.id = Some(self.playlist_id.fetch_add(1, Ordering::Relaxed));
                playlist
            });
        self.playlists.write().unwrap().extend(playlists);
    }

    pub fn add_album(&self, album: &mut Album) {
        album.id = Some(self.album_id.fetch_add(1, Ordering::Relaxed));
        self.albums.write().unwrap().push(album.clone());
    }

    pub fn add_artist(&self, artist: &mut Artist) {
        artist.id = Some(self.artist_id.fetch_add(1, Ordering::Relaxed));
        self.artists.write().unwrap().push(artist.clone());
    }

    pub fn add_playlist(&self, playlist: &Playlist) {
        let p = Playlist {
            id: Some(self.playlist_id.fetch_add(1, Ordering::Relaxed)),
            ..playlist.clone()
        };
        self.playlists.write().unwrap().push(p);
    }

    pub fn add_track(&self, track: &Track) {
        let t = Track {
            id: Some(self.track_id.fetch_add(1, Ordering::Relaxed)),
            ..track.clone()
        };
        self.tracks.write().unwrap().push(t);
    }

    pub fn sync_artist(&self, artist: &mut Artist) {
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
    }

    pub fn sync_album(&self, album: &mut Album) {
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
    }

    pub fn sync_tracks(&self, tracks: &mut Vec<Track>) {
        tracks
            .iter()
            .filter(|track| {
                let tracks = self.tracks.read().unwrap();
                tracks
                    .iter()
                    .find(|t| t.uri == track.uri)
                    .map(|_t| false)
                    .unwrap_or(true)
            })
            .for_each(|t| self.add_track(t))
    }

    pub fn sync_playlists(&self, playlists: &mut Vec<Playlist>) {
        playlists
            .iter()
            .filter(|playlist| {
                let playlists = self.playlists.read().unwrap();
                playlists
                    .iter()
                    .find(|p| p.uri == playlist.uri)
                    .map(|_p| false)
                    .unwrap_or(true)
            })
            .for_each(|p| self.add_playlist(p))
    }

    pub fn search(&self, query: &'static str) -> Vec<Track> {
        self.tracks
            .read()
            .unwrap()
            .iter()
            .cloned()
            .filter(|track| track.title.contains(query))
            .collect()
    }
}