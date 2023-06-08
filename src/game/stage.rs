use super::*;
use super::car::*;
use super::track::*;
use super::score::*;
use super::car_controller::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(PartialEq)]
pub enum GameStatus {
    Pause,
    Running,
    Over,
}

pub trait Stage {
    fn update(&mut self, delta: f64);
    fn draw(&self, renderer: &Renderer);
}

pub struct GameStage {
    pub round: i64,
    pub(crate) objs: Vec<Option<Box<dyn GameObject>>>,
    status: GameStatus,
}

impl GameStage {
    pub async fn new(keyboard_state: Rc<RefCell<KeyboardState>>) -> Box<Self> {
        let mut objs: Vec<Option<Box<dyn GameObject>>> = vec![];
        objs.push(Some(Box::new(GameStage::gen_track().await)));
        objs.push(Some(Box::new(Car::new(400.0, 80.0, CarType::No8, KeyController::new(keyboard_state.clone())).await)));
        Box::new(GameStage {
            round: 1,
            status: GameStatus::Running,
            objs,
        })
    }

    pub fn find<T: GameObject>(&self) -> Vec<&T> {
        let mut found = vec![];
        for obj in self.objs.iter() {
            if let Some(obj) = obj {
                if let Some(t) = obj.as_any().downcast_ref::<T>() {
                    found.push(t)
                }
            }
        }
        return found;
    }

    pub fn find_mut<T: GameObject>(&mut self) -> Vec<&mut T> {
        let mut objs = vec![];
        for obj in self.objs.iter_mut() {
            if let Some(obj) = obj {
                if let Some(t) = obj.as_mut_any().downcast_mut::<T>() {
                    objs.push(t)
                }
            }
        }
        return objs;
    }

    fn reset_if_all_dead(&mut self) {
        let cars = self.find_mut::<car::Car>();
        let some_alive = cars.iter().any(|car| {
            car.status == CarStatus::Live
        });
        if !some_alive {
            self.round += 1;
            for car in self.find_mut::<car::Car>().iter_mut() {
                car.reset(400.0, 80.0, 0.0);
                car.status = CarStatus::Live;
            }
        }
    }


    pub async fn gen_track() -> Track {
        return Track::new(0.0, 0.0, vec![
            TrackSegmentDirection::BottomToRight,
            TrackSegmentDirection::LeftToRight,
            TrackSegmentDirection::LeftToRight,
            TrackSegmentDirection::LeftToRight,
            TrackSegmentDirection::LeftToRight,
            TrackSegmentDirection::LeftToBottom,
            TrackSegmentDirection::TopToBottom,
            TrackSegmentDirection::TopToLeft,
            TrackSegmentDirection::RightToLeft,
            TrackSegmentDirection::RightToLeft,
            TrackSegmentDirection::RightToBottom,
            TrackSegmentDirection::TopToRight,
            TrackSegmentDirection::LeftToRight,
            TrackSegmentDirection::LeftToRight,
            TrackSegmentDirection::LeftToRight,
            TrackSegmentDirection::LeftToRight,
            TrackSegmentDirection::LeftToRight,
            TrackSegmentDirection::LeftToBottom,
            TrackSegmentDirection::TopToLeft,
            TrackSegmentDirection::RightToLeft,
            TrackSegmentDirection::RightToLeft,
            TrackSegmentDirection::RightToLeft,
            TrackSegmentDirection::RightToLeft,
            TrackSegmentDirection::RightToLeft,
            TrackSegmentDirection::RightToLeft,
            TrackSegmentDirection::RightToLeft,
            TrackSegmentDirection::RightToTop,
            TrackSegmentDirection::BottomToTop,
            TrackSegmentDirection::BottomToTop,
            TrackSegmentDirection::BottomToTop,
        ]).await
    }
}

impl Stage for GameStage {
    fn draw(&self, renderer: &Renderer) {
        for obj in self.objs.iter() {
            match obj {
                Some(obj) => {
                    obj.draw(renderer);
                },
                None => {},
            };
        }
    }

    fn update(&mut self, delta: f64) {
        self.reset_if_all_dead();
        for i in 1..self.objs.len() {
            let mut obj = std::mem::replace(&mut self.objs[i], None);
            match obj {
                Some(ref mut obj) => {
                    obj.update(self, delta);
                },
                _ => {}
            }
            self.objs[i] = obj;
        }
    }
}
