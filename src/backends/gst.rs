use super::player::*;
use super::state::PlayerState;
use library::Track;
use failure::{Error, err_msg};
use std::time::Duration;
use channel::{self, Sender, Receiver};
use gstreamer::{self as gst, prelude::*, MessageView, StateChangeReturn};
use std::sync::{atomic, Mutex, Arc};
use std::thread;

#[derive(Debug)]
pub struct GstBackend {
    queue: Mutex<Vec<Track>>,
    current_index: atomic::AtomicUsize,
    current_track: Mutex<Option<Track>>,
    current_volume: atomic::AtomicUsize,
    state: Mutex<PlayerState>,
    blend_time: Mutex<Duration>,
    pipeline: gst::Pipeline,
    decoder: gst::Element,
    volume: gst::Element,
    sink: gst::Element,
    tx: Sender<PlayerEvent>,
    rx: Receiver<PlayerEvent>
}

impl GstBackend {
    fn new() -> Result<Arc<GstBackend>, Error> {
        let pipeline = gst::Pipeline::new(None);
        let decoder = gst::ElementFactory::make("uridecodebin", None)
            .ok_or_else(|| err_msg("can't build uridecodebin"))?;
        let volume = gst::ElementFactory::make("volume", None)
            .ok_or_else(|| err_msg("can't build volume"))?;
        let sink = gst::ElementFactory::make("autoaudiosink", None)
            .ok_or_else(|| err_msg("can't build autoaudiosink"))?;
        let (tx, rx) = channel::unbounded();
        let backend = GstBackend {
            queue: Mutex::new(vec![]),
            current_index: atomic::AtomicUsize::new(0),
            current_track: Mutex::new(None),
            blend_time: Mutex::new(Duration::default()),
            current_volume: atomic::AtomicUsize::new(100),
            state: Mutex::new(PlayerState::Stop),
            pipeline,
            decoder,
            volume,
            sink,
            tx,
            rx
        };

        backend.pipeline.add(&backend.decoder)?;
        backend.pipeline.add(&backend.volume)?;
        backend.pipeline.add(&backend.sink)?;

        backend.volume.link(&backend.sink)?;

        let sink_pad = backend.volume.get_static_pad("sink").ok_or_else(|| err_msg("missing sink pad on volume element"))?;
        backend.decoder.connect_pad_added(move |_el: &gst::Element, pad: &gst::Pad| {
            pad.link(&sink_pad);
        });

        let backend = Arc::new(backend);

        {
            let backend = Arc::clone(&backend);
            thread::spawn(move|| {
                if let Some(bus) = backend.pipeline.get_bus() {
                    let res: Result<(), Error> = match bus.pop() {
                        None => Ok(()),
                        Some(msg) => {
                            match msg.view() {
                                MessageView::Eos(..) => {
                                    println!("eos");
                                    let backend = Arc::clone(&backend);
                                    //backend.next()?;
                                    Ok(())
                                },
                                MessageView::Error(err) => {
                                    bail!(
                                        "Error from {}: {} ({:?})",
                                        msg.get_src().unwrap().get_path_string(),
                                        err.get_error(),
                                        err.get_debug()
                                    );
                                },
                                _ => Ok(()),
                            }
                        },
                    };
                }
                Ok(())
            });
        }

        Ok(backend)
    }

    fn set_track(&mut self, track: &Track) -> Result<(), Error> {
        debug!("Selecting {:?}", track);
        if let StateChangeReturn::Failure = self.pipeline.set_state(gst::State::Null) {
            bail!("can't stop pipeline")
        }
        self.decoder.set_property_from_str("uri", track.stream_url.as_str());

        let state = match *self.state.lock().unwrap() {
            PlayerState::Play => gst::State::Playing,
            PlayerState::Pause => gst::State::Paused,
            PlayerState::Stop => gst::State::Null
        };

        if let StateChangeReturn::Failure = self.pipeline.set_state(state) {
            bail!("can't restart pipeline")
        }
        Ok(())
    }
}
/*
impl PlayerBackend for GstBackend {
    fn enqueue(&mut self, track: &Track) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(track.clone());
    }

    fn enqueue_multiple(&mut self, tracks: &[Track]) {
        let mut queue = self.queue.lock().unwrap();
        queue.append(&mut tracks.to_vec());
    }

    fn play_next(&mut self, track: &Track) {
        let current_index = self.current_index.load(atomic::Ordering::Relaxed);
        let mut queue = self.queue.lock().unwrap();
        queue.insert(current_index + 1, track.clone());
    }

    fn queue(&self) -> Vec<Track> {
        self.queue.lock().unwrap().clone()
    }

    fn clear_queue(&mut self) {
        self.queue.lock().unwrap().clear();
        self.current_index.store(0, atomic::Ordering::Relaxed);
    }

    fn current(&self) -> Option<Track> {
        self.current_track.lock().unwrap().clone()
    }

    fn prev(&mut self) -> Result<Option<()>, Error> {
        let current_index = self.current_index.load(atomic::Ordering::Relaxed);
        if current_index == 0 {
            self.set_state(PlayerState::Stop)?;
            return Ok(None);
        }
        self.current_index.store(current_index - 1, atomic::Ordering::Relaxed);
        let current_track = self.current_track.lock().unwrap();
        *current_track = self.queue.lock().unwrap().get(current_index - 1).cloned();
        if let Some(track) = current_track.clone() {
            self.set_track(&track)?;
            Ok(Some(()))
        }else {
            Ok(None)
        }
    }

    fn next(&mut self) -> Result<Option<()>, Error> {
        let current_index = self.current_index.load(atomic::Ordering::Relaxed);
        if current_index >= self.queue.lock().unwrap().len() {
            self.set_state(PlayerState::Stop)?;
            return Ok(None);
        }
        self.current_index.store(current_index + 1, atomic::Ordering::Relaxed);
        let current_track = self.current_track.lock().unwrap();
        *current_track = self.queue.lock().unwrap().get(current_index + 1).cloned();
        if let Some(track) = current_track.clone() {
            self.set_track(&track)?;
            Ok(Some(()))
        }else {
            Ok(None)
        }
    }

    fn set_state(&mut self, new_state: PlayerState) -> Result<(), Error> {
        if let StateChangeReturn::Failure = self.pipeline.set_state(new_state.clone().into()) {
            bail!("can't play pipeline")
        }
        let mut state = self.state.lock().unwrap();
        *state = new_state;
        Ok(())
    }

    fn state(&self) -> PlayerState {
        self.state.lock().unwrap().clone()
    }

    fn set_volume(&mut self, volume: usize) -> Result<(), Error> {
        unimplemented!()
    }

    fn volume(&self) -> usize {
        self.current_volume.load(atomic::Ordering::Relaxed)
    }

    fn set_blend_time(&mut self, duration: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    fn blend_time(&self) -> Duration {
        self.blend_time.lock().unwrap().clone()
    }

    fn seek(&mut self, duration: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    fn observe(&self) -> Receiver<PlayerEvent> {
        unimplemented!()
    }
}*/