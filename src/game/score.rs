use std::collections::HashMap;
use super::*;

const TOP_LEFT: FVec = FVec::new(0.0,0.0);
const TOP_RIGHT: FVec = FVec::new(1.0,0.0);
const BOTTOM_RIGHT: FVec = FVec::new(1.0,1.0);
const BOTTOM_LEFT: FVec = FVec::new(0.0,1.0);

const TOP_TO_LEFT: FVec = BOTTOM_LEFT - TOP_TO_RIGHT;
const TOP_TO_BOTTOM: FVec = BOTTOM_LEFT - TOP_LEFT;
const TOP_TO_RIGHT: FVec = BOTTOM_RIGHT - TOP_LEFT;

const RIGHT_TO_TOP: FVec = TOP_LEFT - BOTTOM_RIGHT;
const RIGHT_TO_LEFT: FVec = TOP_LEFT - TOP_RIGHT;
const RIGHT_TO_BOTTOM: FVec = BOTTOM_LEFT - TOP_RIGHT;

const BOTTOM_TO_LEFT: FVec = TOP_LEFT - BOTTOM_RIGHT;
const BOTTOM_TO_TOP: FVec = BOTTOM_LEFT - TOP_LEFT;
const BOTTOM_TO_RIGHT: FVec = TOP_RIGHT - BOTTOM_LEFT;

const LEFT_TO_TOP: FVec = TOP_RIGHT - BOTTOM_LEFT;
const LEFT_TO_RIGHT: FVec = TOP_RIGHT - TOP_LEFT;
const LEFT_TO_BOTTOM: FVec = BOTTOM_RIGHT - TOP_LEFT;

pub struct Score {
    pub score: f64,
    position: HashMap<uuid::Uuid, FVec>
}

impl Score {
    pub fn new() -> Self {
        Score {
            position: HashMap::new(),
            score: 0.0
        }
    }

    pub fn update(&mut self, car: &car::Car, track: &track::Track) {
        use track::TrackSegmentDirection::*;
        let current_pos = car.body.get_center();
        let mut diff = FVec::new(0.0, 0.0);
        if let Some(seg) = track.on_which_track_seg(car) {
            if let Some(current_pos) = current_pos {
                if let Some(last_pos) = (self.position.get(&seg.id)) {
                    diff = current_pos - *last_pos;
                } 
                self.position.insert(seg.id, current_pos);
            }
            let score = match seg.direction {
                TopToLeft => diff.project_on(&TOP_TO_LEFT).distance(&TOP_LEFT),
                TopToBottom => diff.project_on(&TOP_TO_BOTTOM).distance(&TOP_LEFT),
                TopToRight => diff.project_on(&TOP_TO_RIGHT).distance(&TOP_LEFT),
                RightToTop => diff.project_on(&RIGHT_TO_TOP).distance(&TOP_LEFT),
                RightToLeft => diff.project_on(&RIGHT_TO_LEFT).distance(&TOP_LEFT),
                RightToBottom => diff.project_on(&RIGHT_TO_BOTTOM).distance(&TOP_LEFT),
                BottomToLeft => diff.project_on(&BOTTOM_TO_LEFT).distance(&TOP_LEFT),
                BottomToTop => diff.project_on(&BOTTOM_TO_TOP).distance(&TOP_LEFT),
                BottomToRight => diff.project_on(&BOTTOM_TO_RIGHT).distance(&TOP_LEFT),
                LeftToTop => diff.project_on(&LEFT_TO_TOP).distance(&TOP_LEFT),
                LeftToRight => diff.project_on(&LEFT_TO_RIGHT).distance(&TOP_LEFT),
                LeftToBottom => diff.project_on(&LEFT_TO_BOTTOM).distance(&TOP_LEFT),
            };
            self.score += score;
        }
    }
}
