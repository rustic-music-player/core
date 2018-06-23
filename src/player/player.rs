use gstreamer as gst;
use player::Queue;
use gstreamer::{MessageView, StateChangeReturn};
use std::thread;
use std::time::Duration;
use gstreamer::prelude::*;
use library::Track;
use bus::{SharedBus, Message};
use std::sync::{Arc, Mutex, Condvar};
use failure::{Error, err_msg};

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PlayerState {
    Play,
    Stop,
    Pause
}

impl From<PlayerState> for gst::State {
    fn from(state: PlayerState) -> gst::State {
        match state {
            PlayerState::Play => gst::State::Playing,
            _ => gst::State::Paused
        }
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub state: PlayerState,
    pub queue: Queue,
    backend: GstBackend,
    volume: u32,
    bus: SharedBus
}

pub type SharedPlayer = Arc<Mutex<Player>>;

impl Player {
    pub fn new(bus: SharedBus) -> Result<SharedPlayer, Error> {
        Ok(Arc::new(Mutex::new(Player {
            state: PlayerState::Stop,
            queue: Queue::new(),
            backend: GstBackend::new()?,
            volume: 100,
            bus
        })))
    }

    pub fn play(&mut self) -> Result<(), Error> {
        match self.state {
            PlayerState::Stop => {
                let current = self.queue.current();
                if let Some(track) = current {
                    self.state = PlayerState::Play;
                    self.bus.lock().unwrap().emit(&Message::PlayerState(PlayerState::Play));
                    self.select_track(&track)?;
                }
                Ok(())
            },
            PlayerState::Pause => {
                self.state = PlayerState::Play;
                self.bus.lock().unwrap().emit(&Message::PlayerState(PlayerState::Play));
                self.backend.play()?;
                Ok(())
            },
            _ => Ok(())
        }
    }

    pub fn pause(&mut self) -> Result<(), Error> {
        self.state = PlayerState::Pause;
        self.bus.lock().unwrap().emit(&Message::PlayerState(PlayerState::Pause));
        self.backend.pause()
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        self.state = PlayerState::Stop;
        self.bus.lock().unwrap().emit(&Message::PlayerState(PlayerState::Stop));
        self.bus.lock().unwrap().emit(&Message::CurrentlyPlaying(None));
        self.backend.stop()?;
        self.queue.clear();
        Ok(())
    }

    pub fn prev(&mut self) -> Result<(), Error> {
        {
            if self.queue.prev().is_none() {
                self.state = PlayerState::Stop;
                self.bus.lock().unwrap().emit(&Message::PlayerState(PlayerState::Stop));
                self.bus.lock().unwrap().emit(&Message::CurrentlyPlaying(None));
                self.backend.stop()?;
            }
        }
        if self.state == PlayerState::Play {
            if let Some(track) = self.queue.current() {
                self.select_track(&track)?;
            }
        }
        Ok(())
    }

    pub fn next(&mut self) -> Result<(), Error> {
        {
            if self.queue.next().is_none() {
                self.state = PlayerState::Stop;
                self.bus.lock().unwrap().emit(&Message::PlayerState(PlayerState::Stop));
                self.bus.lock().unwrap().emit(&Message::CurrentlyPlaying(None));
                self.backend.stop()?;
            }
        }
        if self.state == PlayerState::Play {
            if let Some(track) = self.queue.current() {
                self.select_track(&track)?;
            }
        }
        Ok(())
    }

    pub fn volume(&self) -> u32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: u32) -> Result<(), Error> {
        self.volume = volume;
        self.backend.set_volume(f64::from(volume) / 100.0)?;
        self.bus.lock().unwrap().emit(&Message::Volume);
        Ok(())
    }

    fn get_backend(&self) -> &GstBackend {
        &self.backend
    }

    fn select_track(&self, track: &Track) -> Result<(), Error> {
        self.bus.lock().unwrap().emit(&Message::CurrentlyPlaying(Some(track.clone())));
        self.backend.set_track(track, self.state.clone())
    }
}

#[derive(Debug, Clone)]
struct GstBackend {
    pipeline: gst::Pipeline,
    decoder: gst::Element,
    volume: gst::Element,
    sink: gst::Element
}

impl GstBackend {
    fn new() -> Result<GstBackend, Error> {
        gst::init().unwrap();
        let player: GstBackend = GstBackend {
            pipeline: gst::Pipeline::new(None),
            decoder: gst::ElementFactory::make("uridecodebin", None).ok_or_else(|| err_msg("can't build uridecodebin"))?,
            volume: gst::ElementFactory::make("volume", None).ok_or_else(|| err_msg("can't build volume"))?,
            sink: gst::ElementFactory::make("autoaudiosink", None).ok_or_else(|| err_msg("can't build autoaudiosink"))?
        };

        player.pipeline.add(&player.decoder)?;
        player.pipeline.add(&player.volume)?;
        player.pipeline.add(&player.sink)?;

        player.volume.link(&player.sink)?;

        let sink_pad = player.volume.get_static_pad("sink").ok_or_else(|| err_msg("missing sink pad on volume element"))?;
        player.decoder.connect_pad_added(move |_el: &gst::Element, pad: &gst::Pad| {
            pad.link(&sink_pad);
        });

        Ok(player)
    }

    fn get_bus(&self) -> Result<gst::Bus, Error> {
        self.pipeline.get_bus().ok_or_else(|| err_msg("missing bus"))
    }

    fn set_track(&self, track: &Track, state: PlayerState) -> Result<(), Error> {
        debug!("Selecting {:?}", track);
        if let StateChangeReturn::Failure = self.pipeline.set_state(gst::State::Null) {
            bail!("can't stop pipeline")
        }
        self.decoder.set_property_from_str("uri", track.stream_url.as_str());

        if let StateChangeReturn::Failure = self.pipeline.set_state(state.into()) {
            bail!("can't restart pipeline")
        }

        Ok(())
    }

    fn set_volume(&self, volume: f64) -> Result<(), Error> {
        debug!("set volume {}", volume);
        self.volume.set_property("volume", &volume)?;
        Ok(())
    }

    fn play(&self) -> Result<(), Error> {
        if let StateChangeReturn::Failure = self.pipeline.set_state(gst::State::Playing) {
            bail!("can't play pipeline")
        }
        Ok(())
    }

    fn pause(&self) -> Result<(), Error> {
        if let StateChangeReturn::Failure = self.pipeline.set_state(gst::State::Paused) {
            bail!("can't pause pipeline")
        }
        Ok(())
    }

    fn stop(&self) -> Result<(), Error> {
        if let StateChangeReturn::Failure = self.pipeline.set_state(gst::State::Null) {
            bail!("can't stop pipeline")
        }
        Ok(())
    }
}

pub fn main_loop(player: SharedPlayer, running: Arc<(Mutex<bool>, Condvar)>) -> Result<thread::JoinHandle<()>, Error> {
    thread::Builder::new()
        .name("GStreamer Backend".into())
        .spawn(move|| {
            info!("Starting GStreamer Backend");
            let &(ref lock, ref cvar) = &*running;
            let mut keep_running = lock.lock().unwrap();
            while *keep_running {
                {
                    let mut player = player.lock().unwrap();
                    if let Ok(bus) = player.get_backend().get_bus() {
                        match bus.pop() {
                            None => Ok(()),
                            Some(msg) => {
                                match msg.view() {
                                    MessageView::Eos(..) => player.next(),
                                    MessageView::Error(err) => {
                                        println!(
                                            "Error from {}: {} ({:?})",
                                            msg.get_src().unwrap().get_path_string(),
                                            err.get_error(),
                                            err.get_debug()
                                        );
                                        break;
                                    },
                                    _ => Ok(()),
                                }
                            },
                        }.unwrap()
                    }
                }
                let result = cvar.wait_timeout(keep_running, Duration::from_millis(100)).unwrap();
                keep_running = result.0;
            }
            info!("Stopping GStreamer Backend");
            { // Cleanup
                let mut player = player.lock().unwrap();
                player.stop().unwrap();
            }
            info!("GStreamer Backend stopped");
        })
        .map_err(Error::from)
}