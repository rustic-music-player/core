use rodio;
use library::Track;
use failure::Error;
use super::player::*;
use super::state::PlayerState;
use std::time::Duration;
use channel::{self, Receiver, Sender};
use std::sync::Arc;

pub struct RodioBackend {
    queue: Vec<Track>,
    current_index: usize,
    current_track: Option<Track>,
    current_volume: usize,
    state: PlayerState,
    blend_time: Duration,
    device: rodio::Device,
    tx: Sender<PlayerEvent>,
    rx: Receiver<PlayerEvent>
}

impl RodioBackend {
    pub fn new() -> Result<Arc<RodioBackend>, Error> {
        let device = rodio::default_output_device().unwrap();
        let (tx, rx) = channel::unbounded();
        let backend = RodioBackend {
            queue: vec![],
            current_index: 0,
            current_track: None,
            current_volume: 0,
            state: PlayerState::Stop,
            blend_time: Duration::default(),
            device,
            tx,
            rx
        };

        Ok(Arc::new(backend))
    }
}

impl PlayerBackend for RodioBackend {
    fn enqueue(&mut self, track: &Track) {
        self.queue.push(track.clone());
    }

    fn enqueue_multiple(&mut self, tracks: &[Track]) {
        self.queue.append(&mut tracks.to_vec());
    }

    fn play_next(&mut self, track: &Track) {
        unimplemented!()
    }

    fn queue(&self) -> Vec<Track> {
        unimplemented!()
    }

    fn clear_queue(&mut self) {
        unimplemented!()
    }

    fn current(&self) -> Option<Track> {
        unimplemented!()
    }

    fn prev(&mut self) -> Result<Option<()>, Error> {
        unimplemented!()
    }

    fn next(&mut self) -> Result<Option<()>, Error> {
        unimplemented!()
    }

    fn set_state(&mut self, state: PlayerState) -> Result<(), Error> {
        unimplemented!()
    }

    fn state(&self) -> PlayerState {
        unimplemented!()
    }

    fn set_volume(&mut self, volume: usize) -> Result<(), Error> {
        unimplemented!()
    }

    fn volume(&self) -> usize {
        unimplemented!()
    }

    fn set_blend_time(&mut self, duration: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    fn blend_time(&self) -> Duration {
        unimplemented!()
    }

    fn seek(&mut self, duration: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    fn observe(&self) -> Receiver<PlayerEvent> {
        unimplemented!()
    }
}