use library::Track;

#[derive(Debug, Clone)]
pub struct Queue {
    pub tracks: Vec<Track>,
    current: Option<i64>
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            tracks: vec![],
            current: Some(0)
        }
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    pub fn add_multiple(&mut self, tracks: Vec<Track>) {
        self.tracks.append(&mut tracks.to_vec());
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
    }

    pub fn prev(&mut self) -> Option<&Track> {
        self.current
            .and_then(move|index| {
                let prev_index = index - 1;
                if prev_index < 0 {
                    return None;
                }
                self.current = Some(prev_index);
                self.tracks.get(prev_index as usize)
            })
    }

    pub fn next(&mut self) -> Option<&Track> {
        self.current
            .and_then(move|index| {
                let next_index = index + 1;
                if next_index >= self.tracks.len() as i64 {
                    return None;
                }
                self.current = Some(next_index);
                self.tracks.get(next_index as usize)
            })
    }

    pub fn current(&self) -> Option<&Track> {
        self.current
            .and_then(move|index| self.tracks.get(index as usize))
    }

    pub fn size(&self) -> usize {
        self.tracks.len()
    }
}