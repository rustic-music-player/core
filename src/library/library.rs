use library::{Artist, Album, Track, Playlist};
use provider::SharedProviders;
use std::sync::{RwLock, Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use url::Url;

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

    pub fn resolve_track(&self, providers: SharedProviders, uri: &String) -> Option<Track> {
        self.tracks
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|track| &track.uri == uri)
            .or_else(|| {
                Url::parse(uri)
                    .ok()
                    .and_then(|uri| providers
                        .iter()
                        .find(|provider| provider.read().unwrap().uri_scheme() == uri.scheme()))
                    .and_then(|provider| provider.read().unwrap().resolve_track(uri))
            })
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

    pub fn add_playlist(&self, playlist: &mut Playlist) {
        playlist.id = Some(self.playlist_id.fetch_add(1, Ordering::Relaxed));
        self.playlists.write().unwrap().push(playlist.clone());
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