use std::sync::{Arc, Mutex};
use std::fmt;
use library::Track;
use player::state::PlayerState;

pub type SharedBus = Arc<Mutex<MessageBus>>;

type Subscriber = Fn(Message) -> () + Send + Sync + 'static;

#[derive(Debug, Clone, Serialize)]
pub enum Message {
    Volume,
    PlayerState(PlayerState),
    CurrentlyPlaying(Option<Track>),
    Queue,
    Playlist
}

pub struct MessageBus {
    subscriptions: Vec<Box<Subscriber>>
}

impl MessageBus {
    pub fn new() -> SharedBus {
        Arc::new(Mutex::new(MessageBus {
            subscriptions: vec![]
        }))
    }

    pub fn emit(&self, msg: &Message) {
        debug!("Emitting {:?}", msg);
        for subscription in &self.subscriptions {
            subscription(msg.clone());
        }
    }

    pub fn subscribe(&mut self, callback: Box<Subscriber>) {
        self.subscriptions.push(callback);
    }
}

impl fmt::Debug for MessageBus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} Subscriptions", self.subscriptions.len())
    }
}