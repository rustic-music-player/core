use std::fs::{create_dir_all, OpenOptions};
use std::sync::{Arc, RwLock};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::path::Path;
use failure::Error;
use md5;
use reqwest::get;
use image;
use Rustic;

const THUMBNAIL_SIZE: u32 = 512;
const SERVICE_INTERVAL: u64 = 30;

#[derive(Debug)]
struct CachedEntry {
    uri: String,
    filename: String
}

#[derive(Debug, Default)]
pub struct Cache {
    pub coverart: Arc<RwLock<HashMap<String, String>>>
}

pub type SharedCache = Arc<Cache>;

pub fn start(app: Arc<Rustic>) -> Result<thread::JoinHandle<()>, Error> {
    create_dir_all(".cache/coverart")?;

    let handle = thread::spawn(move || {
        loop {
            info!("Caching Coverart...");
            let result: Result<Vec<CachedEntry>, Error> = app.library
                .tracks
                .read()
                .unwrap()
                .iter()
                .filter(|track| track.image_url.is_some())
                .filter(|track| {
                    let map = app.cache.coverart.read().unwrap();
                    !map.contains_key(&track.uri)
                })
                .map(|track| track.image_url.clone().unwrap())
                .map(cache_coverart)
                .collect();

            match result {
                Ok(entries) => {
                    info!("Cached {} images", entries.len());
                    let mut map = app.cache.coverart.write().unwrap();
                    for entry in entries {
                        map.insert(entry.uri, entry.filename);
                    }
                },
                Err(e) => error!("Error: {:?}", e)
            }

            thread::sleep(Duration::new(SERVICE_INTERVAL, 0));
        }
    });
    Ok(handle)
}

fn cache_coverart(uri: String) -> Result<CachedEntry, Error> {
    let base = ".cache/coverart";
    let hash = md5::compute(&uri);
    let filename = format!("{:x}.png", hash);
    let path = format!("{}/{}", base, filename);
    if Path::new(&path).exists() {
        return Ok(CachedEntry {
            filename,
            uri
        });
    }

    debug!("{} -> {}", &uri, &filename);

    let buffer = {
        let mut buffer = Vec::new();
        let mut res = get(&uri)?;
        res.read_to_end(&mut buffer)?;
        buffer
    };
    let img = image::load_from_memory(&buffer)?;
    let thumb = img.thumbnail(THUMBNAIL_SIZE, THUMBNAIL_SIZE);
    let mut file = OpenOptions::new().create(true).write(true).open(&path)?;
    thumb.write_to(&mut file, image::ImageFormat::PNG)?;
    Ok(CachedEntry {
        filename,
        uri
    })
}

impl Cache {
    pub fn new() -> Cache {
        Cache::default()
    }

    pub fn fetch_coverart(&self, uri: String) -> Result<String, Error> {
        {
            let map = self.coverart.read().unwrap();
            if map.contains_key(&uri) {
                return Ok(format!("/cache/coverart/{}", map.get(&uri).unwrap()));
            }
        }
        let entry = cache_coverart(uri)?;
        {
            let mut map = self.coverart.write().unwrap();
            map.insert(entry.uri, entry.filename.clone());
        }
        Ok(format!("/cache/coverart/{}", entry.filename))
    }
}