use gstreamer as gst;

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PlayerState {
    Play,
    Stop,
    Pause,
}

impl Default for PlayerState {
    fn default() -> PlayerState {
        PlayerState::Stop
    }
}

impl From<PlayerState> for gst::State {
    fn from(state: PlayerState) -> gst::State {
        match state {
            PlayerState::Play => gst::State::Playing,
            PlayerState::Pause => gst::State::Paused,
            PlayerState::Stop => gst::State::Null
        }
    }
}