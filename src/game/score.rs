use std::collections::HashMap;
use super::*;

const UP: FVec = FVec::new(0.0, -1.0);
const UP_RIGHT: FVec = FVec::new(1.0, -1.0);
const RIGHT: FVec = FVec::new(1.0, 0.0);
const DOWN_RIGHT: FVec = FVec::new(1.0, 1.0);
const DOWN: FVec = FVec::new(0.0, 1.0);
const DOWN_LEFT: FVec = FVec::new(-1.0, 1.0);
const LEFT: FVec = FVec::new(-1.0, 0.0);
const UP_LEFT: FVec = FVec::new(-1.0, -1.0);

#[derive(Debug)]
pub struct Score {
    pub top_score: f64,
    pub score: f64,
    position: HashMap<uuid::Uuid, FVec>,
    stale_time: f64
}

impl Score {
    pub fn new() -> Self {
        Score {
            position: HashMap::new(),
            score: 0.0,
            top_score: 0.0,
            stale_time: 0.0
        }
    }

    pub fn is_stale_for(&self, time: f64) -> bool {
        self.stale_time > time
    }

    pub fn reset(&mut self) {
        self.position.clear();
        self.top_score = 0.0;
        self.score = 0.0;
        self.stale_time = 0.0
    }

    pub fn update(&mut self, car_body: &BoundingBox, track: &track::Track, delta: f64) {
        use track::TrackSegmentDirection::*;
        let current_pos = car_body.get_center();
        let mut diff = FVec::new(0.0, 0.0);
        if let Some(seg) = track.on_which_track_seg(car_body) {
            if let Some(current_pos) = current_pos {
                if let Some(last_pos) = (self.position.get(&seg.id)) {
                    diff = current_pos - *last_pos;
                } 
                self.position.insert(seg.id, current_pos);
            }
            let update_score = match seg.direction {
                Up => diff.dot(&UP),
                UpRightRight => diff.dot(&UP_RIGHT),
                UpRightUp => diff.dot(&UP_RIGHT),
                Right => diff.dot(&RIGHT),
                DownRightRight => diff.dot(&DOWN_RIGHT),
                DownRightDown => diff.dot(&DOWN_RIGHT),
                Down => diff.dot(&DOWN),
                DownLeftLeft => diff.dot(&DOWN_LEFT),
                DownLeftDown => diff.dot(&DOWN_LEFT),
                Left => diff.dot(&LEFT),
                UpLeftLeft => diff.dot(&UP_LEFT),
                UpLeftUp => diff.dot(&UP_LEFT),
            };
            self.score += update_score;
            if self.score > self.top_score {
                self.top_score = self.score;
                self.stale_time = 0.0;
            } else {
                self.stale_time += delta;
            }
        }
    }
}
