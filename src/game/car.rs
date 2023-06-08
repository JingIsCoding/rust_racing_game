use crate::{engine::*, browser::load_image};
use web_sys::*;
use super::*;
use super::car_controller::*;
use super::car_sensor::*;
use super::score::Score;
use std::hash::{Hash, Hasher};

pub struct Action<T>(T);

const ACCELARATE: f64 = 10.0;
const TURNING_ANGLE: f64 = 3.0;

#[derive(PartialEq, Debug)]
pub enum CarStatus {
    Live,
    Dead,
}

pub enum CarType {
    No5,
    No8
}

pub struct Car {
    pub id: uuid::Uuid,
    pub body: BoundingBox,
    pub status: CarStatus,
    pub turning_angle: f64,
    pub sensor: Sensor,
    image: HtmlImageElement,
    actions: Vec<Action<Movement>>,
    velocity: f64,
    controller: Box<dyn CarController>,
    score: score::Score,
}

const CAR_WIDTH: f64 = 63.0;
const CAR_HEIGHT: f64 = 38.0;

pub enum Movement {
    NotTurning,
    Left,
    Right,
    NotAccelarate,
    Forward,
    Backward,
}

impl Car {
    pub async fn new<T: CarController + 'static>(x: f64, y: f64, car_type: CarType, controller: T) -> Self {
        let image_src = match car_type {
            CarType::No5 => "car_5.png",
            CarType::No8 => "car_8.png",
        };
        let image = load_image(image_src).await.unwrap();
        Car {
            id: uuid::Uuid::new_v4(),
            image,
            status: CarStatus::Live,
            actions: vec![],
            sensor: Sensor::new(),
            turning_angle: 0.0,
            velocity: 0.0,
            body: BoundingBox::new_with_origin(&Rect { x, y, w: CAR_WIDTH, h: CAR_HEIGHT }, FVec { x: 0.0, y: CAR_HEIGHT / 2.0 }),
            score: Score::new(),
            controller: Box::new(controller),
        }
    }

    pub fn reset(&mut self, x: f64, y: f64, rotate: f64) {
        self.velocity = 0.0;
        self.turning_angle = 0.0;
        self.body.reset_to(x, y, rotate);
    }

    fn process_actions(&mut self, delta: f64) {
        for i in 0..self.actions.len() {
            match self.actions[i].0 {
                Movement::Left => {
                    self.turning_angle = -TURNING_ANGLE;
                },
                Movement::Right => {
                    self.turning_angle = TURNING_ANGLE;
                },
                Movement::NotTurning => {
                    self.turning_angle = 0.0;
                },
                Movement::Forward => {
                    self.calculate_pos(ACCELARATE, delta)
                },
                Movement::Backward => {
                    self.calculate_pos(-ACCELARATE * 2.0,delta)
                },
                Movement::NotAccelarate => {
                    self.calculate_pos(0.0, delta)
                }
            }
        }
        self.actions.clear();
    }

    fn calculate_pos(&mut self, accelarating: f64, delta: f64) {
        if self.velocity.abs() > 2.0 {
            if self.velocity < 0.0 {
                self.body.turn_at(-self.turning_angle * delta);
            } else {
                self.body.turn_at(self.turning_angle * delta);
            }
        }
        self.velocity += self.cal_accelate(accelarating);
        if accelarating == 0.0 && self.velocity.abs() < 0.2 {
            self.velocity = 0.0;
        }
        self.body.move_at(self.velocity * delta)
    }

    fn cal_accelate(&self, acc: f64) -> f64 {
        let mut friction = 0.0;
        if self.velocity != 0.0 {
            friction = self.velocity / 50.0;
            if self.velocity.abs() < 10.0 {
                friction *= 50.0;
            }
        }
        return acc - friction;
    }

    fn process_collision(&mut self, track: &track::Track) {
        if let Some((collision, track_type)) = track.on_collide(self) {
            match track_type {
                track::TrackSegmentType::FinishLine => {},
                _ => {
                    self.status = CarStatus::Dead;
                }
            }
        }
    }

}

impl GameObject for Car {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

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

    fn update(&mut self, stage: &mut GameStage, delta: f64) {
        self.reset_sensor();
        let stage = stage;
        let tracks = stage.find::<track::Track>();
        if !tracks.is_empty() {
            let track = tracks[0];
            self.detect_dis(track);
            self.process_collision(track);
            self.score.update(self, track);
        }
        match(self.status) {
            CarStatus::Live => {
                for movement in self.controller.next_movements(self) {
                    self.actions.push(Action(movement));
                }
            },
            CarStatus::Dead => {},
        }
        self.process_actions(delta);
    }


}


impl Hash for Car {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Car {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
