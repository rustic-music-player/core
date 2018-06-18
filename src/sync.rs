use failure::Error;
use Rustic;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn start(app: Arc<Rustic>) -> Result<thread::JoinHandle<()>, Error> {
    let handle = thread::Builder::new()
        .name("Background Sync".into())
        .spawn(move || {
            loop {
                let providers = app.providers.clone();
                for provider in providers {
                    let mut provider = provider.write().unwrap();
                    info!("Syncing {} library", provider.title());
                    match provider.sync(Arc::clone(&app.library)) {
                        Ok(result) => info!("Synced {} tracks, {} albums, {} artist and {} playlists from {}", result.tracks, result.albums, result.artists, result.playlists, provider.title()),
                        Err(err) => error!("Error syncing {}: {:?}", provider.title(), err)
                    }
                }
                thread::sleep(Duration::from_secs(5 * 60));
            }
        })?;

    Ok(handle)
}