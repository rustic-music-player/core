mod queue;
mod player;

use super::Rustic;
use std::sync::Arc;
use std::thread;

pub use self::queue::Queue;
pub use self::player::{Player, main_loop, PlayerState, SharedPlayer};

pub fn start(app: Arc<Rustic>) -> thread::JoinHandle<()> {
    let player = Arc::clone(&app.player);
    main_loop(player)
}