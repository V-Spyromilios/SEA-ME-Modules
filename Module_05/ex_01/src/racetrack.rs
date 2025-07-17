#[derive(Debug, Clone)]
pub struct RaceTrack {
    pub length: usize,
    pub finish_line: usize,
}

impl RaceTrack {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            finish_line: length - 1,
        }
    }

    pub fn is_finished(&self, position: usize) -> bool {
        position >= self.finish_line
    }
}
