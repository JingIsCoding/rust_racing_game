use std::f64::consts::PI;

use crate::game::car::*;
use crate::game::track::*;
use crate::engine::*;
use ndarray::arr2;

pub const SENSOR_RANGE: f64 = 200.0;

#[derive(Debug)]
pub struct Sensor {
    forward: Line,
    right: Line,
    back: Line,
    left: Line,
    pub forward_dis: f64,
    pub right_dis: f64,
    pub back_dis: f64,
    pub left_dis: f64,
    pub track_direction: f64
}

impl Sensor {
    pub fn new() -> Self {
        Sensor { 
            forward: Line::default(),
            right: Line::default(),
            back: Line::default(),
            left: Line::default(),
            forward_dis: SENSOR_RANGE,
            right_dis: SENSOR_RANGE,
            back_dis: SENSOR_RANGE,
            left_dis: SENSOR_RANGE,
            track_direction: 0.0
        }
    }
    
    pub fn reset(&mut self, center_point: FVec, rotate: f64) {
        let (cx, cy) = (center_point.x, center_point.y);
        let ps = arr2(&[[0.0, SENSOR_RANGE, 0.0, -SENSOR_RANGE], [SENSOR_RANGE, 0.0, -SENSOR_RANGE, 0.0]]);
        let r = arr2(&[[rotate.cos(), -rotate.sin()], [rotate.sin(), rotate.cos()]]);
        let c = arr2(&[[ cx, cx, cx, cx ], [ cy, cy, cy, cy ]]);
        let pc = &r.dot(&ps) + &c;
        let mut coordinates = vec![];
        for i in 0..4 {
            coordinates.push(FVec { x: pc[[0, i]], y: pc[[1, i]] });
        }
        self.forward.start = center_point;
        self.right.start = center_point;
        self.back.start = center_point;
        self.left.start = center_point;

        self.forward.end = coordinates[0];
        self.right.end = coordinates[1];
        self.back.end = coordinates[2];
        self.left.end = coordinates[3];

        self.forward_dis = SENSOR_RANGE;
        self.right_dis = SENSOR_RANGE;
        self.back_dis = SENSOR_RANGE;
        self.left_dis = SENSOR_RANGE;
    }

    pub fn debug(&self, renderer: &Renderer) {
        renderer.stroke_style("blue");
        renderer.line(&self.forward);
        renderer.line(&self.right);
        renderer.line(&self.back);
        renderer.line(&self.left);
        renderer.text(format!("{}", self.forward_dis).as_str(), self.forward.end);
        renderer.text(format!("{}", self.right_dis).as_str(), self.right.end);
        renderer.text(format!("{}", self.back_dis).as_str(), self.back.end);
        renderer.text(format!("{}", self.left_dis).as_str(), self.left.end);
    }
}

impl Car {
    pub fn reset_sensor(&mut self) {
        let (rotate, center_point )= (self.body.rotate, self.body.get_center());
        if let Some(center_point) = center_point {
            self.sensor.reset(center_point, rotate);
        }
    }

    pub fn detect(&mut self, track: &Track) {
        use crate::game::track::TrackSegmentDirection::*;
        for seg in track.segments.iter() {
            if let (Some(car_center), Some(track_center)) = (self.body.get_center(), seg.body.get_center()) {
                if car_center.distance(&track_center) > SENSOR_RANGE * 2.0 {
                    continue;
                }
                for boundary in &seg.boundaries {
                    if let Some(distance) = self.sensor.forward.distance(boundary) {
                        if distance < self.sensor.forward_dis {
                            self.sensor.forward_dis = distance;
                        }
                    }
                    if let Some(distance) = self.sensor.right.distance(boundary) {
                        if distance < self.sensor.right_dis {
                            self.sensor.right_dis = distance;
                        }
                    }
                    if let Some(distance) = self.sensor.back.distance(boundary) {
                        if distance < self.sensor.back_dis {
                            self.sensor.back_dis = distance;
                        }
                    }
                    if let Some(distance) = self.sensor.left.distance(boundary) {
                        if distance < self.sensor.left_dis {
                            self.sensor.left_dis = distance;
                        }
                    }
                };
                if let Some(seg) = track.on_which_track_seg(&self.body) {
                    self.sensor.track_direction = match seg.direction {
                        Up => 2.0 * PI * 6.0 / 8.0,
                        UpRightRight => 2.0 * PI * 7.0 / 8.0,
                        UpRightUp => 2.0 * PI * 7.0 / 8.0,
                        Right => 0.0,
                        DownRightRight => 2.0 * PI * 1.0 / 8.0,
                        DownRightDown => 2.0 * PI * 1.0 / 8.0,
                        Down => 2.0 * PI * 2.0 / 8.0,
                        DownLeftLeft => 2.0 * PI * 3.0 / 8.0,
                        DownLeftDown => 2.0 * PI * 3.0 / 8.0,
                        Left => 2.0 * PI * 4.0 / 8.0,
                        UpLeftLeft => 2.0 * PI * 5.0 / 8.0,
                        UpLeftUp => 2.0 * PI * 5.0 / 8.0,
                    };
                }
            }
        }
    }
}
