use gstreamer as gst;
use player::Queue;
use gstreamer::MessageView;
use std::thread;
use std::time::Duration;
use gstreamer::prelude::*;
use library::Track;
use logger::logger;
use bus::{SharedBus, Message};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum PlayerState {
    #[serde(rename = "play")]
    Play,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "pause")]
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
    pub fn new(bus: SharedBus) -> SharedPlayer {
        Arc::new(Mutex::new(Player {
            state: PlayerState::Stop,
            queue: Queue::new(),
            backend: GstBackend::new(),
            volume: 100,
            bus
        }))
    }

    pub fn play(&mut self) {
        match self.state {
            PlayerState::Stop => {
                let current = self.queue.current();
                match current {
                    Some(track) => {
                        self.state = PlayerState::Play;
                        self.bus.lock().unwrap().emit(&Message::PlayerState);
                        self.select_track(&track);
                    },
                    None => {}
                }
            },
            PlayerState::Pause => {
                self.state = PlayerState::Play;
                self.bus.lock().unwrap().emit(&Message::PlayerState);
                self.backend.play();
            },
            _ => {}
        }
    }

    pub fn pause(&mut self) {
        self.state = PlayerState::Pause;
        self.bus.lock().unwrap().emit(&Message::PlayerState);
        self.backend.pause();
    }

    pub fn stop(&mut self) {
        self.state = PlayerState::Stop;
        self.bus.lock().unwrap().emit(&Message::PlayerState);
        self.backend.stop();
        self.queue.clear();
    }

    pub fn prev(&mut self) {
        {
            match self.queue.prev() {
                None => {
                    self.state = PlayerState::Stop;
                    self.bus.lock().unwrap().emit(&Message::PlayerState);
                    self.backend.stop();
                },
                _ => {}
            }
        }
        if self.state == PlayerState::Play {
            match self.queue.current() {
                Some(track) => {
                    self.select_track(&track);
                },
                _ => {}
            }
        }
    }

    pub fn next(&mut self) {
        {
            match self.queue.next() {
                None => {
                    self.state = PlayerState::Stop;
                    self.bus.lock().unwrap().emit(&Message::PlayerState);
                },
                _ => {}
            }
        }
        if self.state == PlayerState::Play {
            match self.queue.current() {
                Some(track) => {
                    self.select_track(&track);
                },
                _ => {}
            }
        }
    }

    pub fn volume(&self) -> u32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: u32) {
        self.volume = volume;
        self.backend.set_volume(volume as f64 / 100.0);
        self.bus.lock().unwrap().emit(&Message::Volume);
    }

    fn get_backend(&self) -> &GstBackend {
        &self.backend
    }

    fn select_track(&self, track: &Track) {
        self.backend.set_track(track, self.state.clone());
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
    fn new() -> GstBackend {
        gst::init().unwrap();
        let player = GstBackend {
            pipeline: gst::Pipeline::new(None),
            decoder: gst::ElementFactory::make("uridecodebin", None).expect("uridecodebin"),
            volume: gst::ElementFactory::make("volume", None).expect("volume"),
            sink: gst::ElementFactory::make("autoaudiosink", None).expect("autoaudiosink")
        };

        player.pipeline.add(&player.decoder).expect("add decoder to pipeline");
        player.pipeline.add(&player.volume).expect("add volume to pipeline");
        player.pipeline.add(&player.sink).expect("add sink to pipeline");

        player.volume.link(&player.sink);

        let sink_pad = player.volume.get_static_pad("sink").expect("volume sink_pad");
        player.decoder.connect_pad_added(move |_el: &gst::Element, pad: &gst::Pad| {
            pad.link(&sink_pad);
        });

        player
    }

    fn get_bus(&self) -> gst::Bus {
        self.pipeline.get_bus().unwrap()
    }

    fn set_track(&self, track: &Track, state: PlayerState) {
        debug!(logger, "Selecting {:?}", track);
        self.pipeline.set_state(gst::State::Null);
        self.decoder.set_property_from_str("uri", track.stream_url.as_str());

        let ret = self.pipeline.set_state(state.into());

        assert_ne!(ret, gst::StateChangeReturn::Failure);
    }

    fn set_volume(&self, volume: f64) {
        debug!(logger, "set volume {}", volume);
        match self.volume.set_property("volume", &volume) {
            Err(err) => error!(logger, "Can't set Volume {:?}", err),
            _ => {},
        }
    }

    fn play(&self) {
        self.pipeline.set_state(gst::State::Playing);
    }

    fn pause(&self) {
        self.pipeline.set_state(gst::State::Paused);
    }

    fn stop(&self) {
        self.pipeline.set_state(gst::State::Null);
    }
}

pub fn main_loop(player: SharedPlayer) -> thread::JoinHandle<()> {
    thread::spawn(move|| {
        loop {
            {
                let mut player = player.lock().unwrap();
                let bus = player.get_backend().get_bus();

                match bus.pop() {
                    None => (),
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
                            _ => (),
                        }
                    },
                };
            }
            thread::sleep(Duration::from_millis(100));
        }
    })
}