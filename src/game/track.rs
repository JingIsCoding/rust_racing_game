use web_sys::*;
use std::f64::consts::PI;
use std::hash::{Hash, Hasher};
use crate::{engine::*, browser::load_image};
use super::*;
use uuid;

const TRACK_SEG_WIDTH: f64 = 150.0;
const TRACK_SEG_HEIGHT: f64 = 150.0;

#[derive(Clone, Copy)]
pub enum TrackSegmentDirection {
    TopToLeft,
    TopToBottom,
    TopToRight,

    RightToTop,
    RightToLeft,
    RightToBottom,

    BottomToLeft,
    BottomToTop,
    BottomToRight,

    LeftToTop,
    LeftToRight,
    LeftToBottom
}

#[derive(PartialEq, Clone, Copy)]
pub enum TrackSegmentType {
    UpDown,
    LeftRight,
    LowerLeft,
    LowerRight,
    UpperLeft,
    UpperRight,
    FinishLine,
}

pub struct TrackSegment {
    pub id: uuid::Uuid,
    pub track_type: TrackSegmentType,
    pub direction: TrackSegmentDirection,
    pub finish_line: Option<Line>,
    pub boundaries: Vec<Line>,
    pub body: BoundingBox,
    image: HtmlImageElement,
    debug: bool,
}

impl TrackSegment {
    pub async fn new(x: f64, y: f64, track_type: TrackSegmentType, direction: TrackSegmentDirection) -> Self {
        let (image_src, lines) = match track_type {
            TrackSegmentType::UpDown => ("track_up.png", vec![Line::new(x, y, x, y + 150.0), Line::new(x + 150.0, y, x + 150.0, y + 150.0)]),
            TrackSegmentType::LeftRight => ("track_left.png", vec![Line::new(x, y, x + 150.0, y), Line::new(x, y + 150.0, x + 150.0, y + 150.0)]),
            TrackSegmentType::LowerLeft => ("track_lower_left.png", Self::arc_lines(0.0, 150.0, x + 150.0, y, PI / 2.0, 8)),
            TrackSegmentType::LowerRight => ("track_lower_right.png", Self::arc_lines(150.0, -0.0, x, y, PI / 2.0, 8)),
            TrackSegmentType::UpperLeft => ("track_upper_left.png", Self::arc_lines(-150.0, 0.0, x + 150.0, y + 150.0, PI / 2.0, 8)),
            TrackSegmentType::UpperRight => ("track_upper_right.png", Self::arc_lines(0.0, -150.0, x, y + 150.0, PI / 2.0, 8)),
            TrackSegmentType::FinishLine => ("finish_line.png", vec![]),
        };
        let image = load_image(image_src).await.unwrap();
        return TrackSegment {
            id: uuid::Uuid::new_v4(),
            image,
            track_type,
            direction,
            finish_line: None,
            boundaries: lines,
            body: BoundingBox::new_with_origin(&Rect { x, y, w: TRACK_SEG_WIDTH, h: TRACK_SEG_HEIGHT }, FVec { x: 0.0, y: 0.0}),
            debug: false,
        }
    }

    pub fn set_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        return self;
    }

    fn arc_lines(start_x: f64, start_y: f64, center_x: f64, center_y: f64, total_radis: f64, num_seg: i32) -> Vec<Line> {
        let seg_angle = total_radis / num_seg as f64;
        let mut lines = vec![];
        let mut last_x = start_x;
        let mut last_y = start_y;
        for i in 0..(num_seg + 1) {
            let rotate = seg_angle * i as f64;
            let new_x = start_x * rotate.cos() - start_y * rotate.sin();
            let new_y = start_x * rotate.sin() + start_y * rotate.cos();
            lines.push(Line::new(last_x + center_x, last_y + center_y, new_x + center_x, new_y + center_y));
            last_x = new_x;
            last_y = new_y;
        }
        return lines;
    }

    pub fn collide(&self, other_body: &BoundingBox) -> Option<FVec> {
        let (_, lines) = other_body.get_coordinates();
        if let(Some(center), Some(other_center)) = ( self.body.get_center(), other_body.get_center()) {
            if center.distance(&other_center) > 500.0 {
                return None;
            }
        }
        if let Some(lines) = lines {
            for line in lines {
                for collide_line in &self.boundaries {
                    if let Some(point) = line.intersect(collide_line) {
                        return Some(point);
                    }
                }
            }
        }
        None
    }

    fn debug(&self, renderer: &Renderer) {
        for line in &self.boundaries {
            renderer.line(line);
        }
    }
}

pub struct Track {
    pub segments: Vec<TrackSegment>
}

impl Track {
    pub async fn new (start_x: f64, start_y: f64, dir_and_types: Vec<TrackSegmentDirection>) -> Self {
        let mut current_x = start_x;
        let mut current_y = start_y;
        let mut segments = vec![];
        for dir in dir_and_types.into_iter() {
            let (track_type, x, y) = Self::dir_to_type_and_offset(&dir, current_x, current_y);
            let seg = TrackSegment::new(current_x, current_y, track_type, dir).await;
            segments.push(seg);
            current_x = x;
            current_y = y;
        }
        Track { segments }
    }

    fn dir_to_type_and_offset(dir: &TrackSegmentDirection, current_x: f64, current_y: f64) -> (TrackSegmentType, f64, f64){
        use TrackSegmentDirection::*;
        use TrackSegmentType::*;
        match *dir {
            TopToLeft => (LowerRight, current_x - TRACK_SEG_WIDTH, current_y),
            TopToBottom  => (UpDown, current_x, current_y + TRACK_SEG_HEIGHT),
            TopToRight  => (LowerLeft, current_x + TRACK_SEG_WIDTH, current_y),

            RightToTop => (LowerLeft, current_x, current_y - TRACK_SEG_HEIGHT),
            RightToLeft => (LeftRight, current_x - TRACK_SEG_WIDTH, current_y),
            RightToBottom => (UpperLeft, current_x, current_y + TRACK_SEG_HEIGHT),

            BottomToRight => (UpperLeft, current_x + TRACK_SEG_WIDTH, current_y),
            BottomToTop => (UpDown, current_x, current_y - TRACK_SEG_HEIGHT),
            BottomToLeft => (UpperRight, current_x - TRACK_SEG_WIDTH, current_y),

            LeftToTop => (LowerRight, current_x, current_y - TRACK_SEG_HEIGHT),
            LeftToRight => (LeftRight, current_x + TRACK_SEG_WIDTH, current_y),
            LeftToBottom => (UpperRight, current_x, current_y + TRACK_SEG_HEIGHT)
        }
    }
}

impl Hash for TrackSegment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for TrackSegment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl TrackSegment {
    fn draw(&self, renderer: &Renderer) {
        let x = self.body.rect.x;
        let y = self.body.rect.y;
        let w = self.body.rect.w;
        let h = self.body.rect.h;
        let origin_x = self.body.origin.x;
        let origin_y = self.body.origin.y;
        renderer.save();
        renderer.translate(&FVec { x: x + origin_x, y: y + origin_y });
        renderer.rotate(self.body.rotate);
        renderer.translate(&FVec { x: -x - origin_x, y: -y - origin_y });
        renderer.draw_image_with_dest(&self.image, &Rect { x: x - origin_x, y: y - origin_y, w, h });
        renderer.restore();
    }
}

impl Track {
    pub fn on_collide(&self, car: &car::Car) -> Option<(FVec, TrackSegmentType)> {
        for track_seg in self.segments.iter() {
            if let Some(point) = track_seg.collide(&car.body) {
                return Some((point, track_seg.track_type));
            }
        }
        None
    }

    pub fn on_which_track_seg(&self, car: &car::Car) -> Option<&TrackSegment> {
        for track_seg in self.segments.iter() {
            if let Some(p) = car.body.get_center() {
                if track_seg.body.contains(&p) {
                    return Some(track_seg);
                }
            }
        }
        None
    }
}

impl GameObject for Track {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn draw(&self, renderer: &Renderer) {
        for track_seg in self.segments.iter() {
            track_seg.draw(renderer);
            if track_seg.debug {
                track_seg.debug(renderer);
            }
        }
    }

    fn update(&mut self, stage: &mut GameStage, delta: f64) {
    }
}
