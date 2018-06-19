mod queue;
mod player;

use super::Rustic;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use failure::Error;

pub use self::queue::Queue;
pub use self::player::{Player, main_loop, PlayerState, SharedPlayer};

pub fn start(app: &Arc<Rustic>, running: Arc<(Mutex<bool>, Condvar)>) -> Result<thread::JoinHandle<()>, Error> {
    let player = Arc::clone(&app.player);
    main_loop(player, running)
}