pub mod state;
pub mod event;

use failure::Error;
use std::time::Duration;
use channel::Receiver;
use library::Track;
use std::any::Any;

pub use self::event::PlayerEvent;
pub use self::state::PlayerState;

pub trait PlayerBackend: Send + Sync {
    /// Put a single track at the end of the current queue
    fn enqueue(&mut self, track: &Track);

    /// Put multiple tracks at the end of the current queue
    fn enqueue_multiple(&mut self, tracks: &[Track]);

    /// Queue single track behind the current
    fn play_next(&mut self, track: &Track);

    /// Returns all tracks which are queued up right now
    fn queue(&self) -> Vec<Track>;

    /// Clear the current queue
    /// Does not stop playback
    fn clear_queue(&mut self);

    /// Returns the currently playing track or None when nothing is playing
    fn current(&self) -> Option<Track>;

    /// Play the previous track in the current queue
    fn prev(&mut self) -> Result<Option<()>, Error>;

    /// Play the next track in the current queue
    fn next(&mut self) -> Result<Option<()>, Error>;

    /// Set the player state
    fn set_state(&mut self, state: PlayerState) -> Result<(), Error>;

    /// Get the player state
    fn state(&self) -> PlayerState;

    /// Set the volume of this player
    fn set_volume(&mut self, volume: f32) -> Result<(), Error>;

    /// Get the volume of this player
    fn volume(&self) -> f32;

    /// Set time from the end of the current track when the next track should start playing
    fn set_blend_time(&mut self, duration: Duration) -> Result<(), Error>;

    /// Get time from the end of the current track when the next track should start playing
    fn blend_time(&self) -> Duration;

    /// Seek to a point in the current track
    fn seek(&mut self, duration: Duration) -> Result<(), Error>;

    fn observe(&self) -> Receiver<PlayerEvent>;

    fn as_any(&self) -> &Any;
}