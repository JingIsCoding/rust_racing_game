use super::*;
use super::car::*;
use super::track::*;
use super::car_controller::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::vec;

const NUM_CARS: u32 = 50;

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
    auto_drive: Option<Rc<RefCell<AutoDrive>>>,
}

impl GameStage {
    pub async fn new(keyboard_state: Rc<RefCell<KeyboardState>>) -> Box<Self> {
        let mut objs: Vec<Option<Box<dyn GameObject>>> = vec![];
        objs.push(Some(Box::new(GameStage::gen_track().await)));
        let (auto_drive, mut cars) = Self::auto_drive_cars().await;
        objs.append(&mut cars);
        Box::new(GameStage {
            round: 1,
            status: GameStatus::Running,
            objs,
            auto_drive,
        })
    }

    pub async fn player_drive_car(keyboard_state: Rc<RefCell<KeyboardState>>) -> (Option<Rc<RefCell<AutoDrive>>>, Vec<Option<Box<dyn GameObject>>>) {
        let mut cars: Vec<Option<Box<dyn GameObject>>> = vec![];
        let id = uuid::Uuid::new_v4();
        let controller = Box::new(KeyController::new(id, keyboard_state));
        cars.push(Some(Box::new(Car::new(id, 400.0, 80.0, CarType::No8, controller).await)));
        (None, cars)
    }

    pub async fn auto_drive_cars() -> (Option<Rc<RefCell<AutoDrive>>>, Vec<Option<Box<dyn GameObject>>>) {
        let auto_drive = Rc::new(RefCell::new(AutoDrive::new(0.3)));
        let mut cars: Vec<Option<Box<dyn GameObject>>> = vec![];
        for _ in 0..NUM_CARS {
            let id = uuid::Uuid::new_v4();
            let controller = Box::new(AutoDriveController::new(id, auto_drive.clone()));
            cars.push(Some(Box::new(Car::new(id, 400.0, 80.0, CarType::No8, controller).await)));
        }
        auto_drive.borrow_mut().new_network().await;
        (Some(auto_drive), cars)
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
        let mut found = vec![];
        for obj in self.objs.iter_mut() {
            if let Some(obj) = obj {
                if let Some(t) = obj.as_mut_any().downcast_mut::<T>() {
                    found.push(t)
                }
            }
        }
        return found;
    }

    fn reset_if_all_dead(&mut self) {
        let cars = self.find_mut::<car::Car>();
        let some_alive = cars.iter().any(|car| {
            car.status == CarStatus::Live
        });
        if !some_alive {
            self.round += 1;
            if let Some(ref auto_drive) = self.auto_drive {
                auto_drive.borrow_mut().next_gen(self.find::<car::Car>());
            }
            for car in self.find_mut::<car::Car>() {
                car.reset(&FVec::new(400.0, 80.0), 0.0);
            }
        }
    }

    pub async fn gen_track() -> Track {
        return Track::new(0.0, 0.0, vec![
            TrackSegmentDirection::UpRightRight,
            TrackSegmentDirection::Right,
            TrackSegmentDirection::Right,
            TrackSegmentDirection::Right,
            TrackSegmentDirection::Right,
            TrackSegmentDirection::DownRightDown,
            TrackSegmentDirection::Down,
            TrackSegmentDirection::DownLeftLeft,
            TrackSegmentDirection::Left,
            TrackSegmentDirection::Left,
            TrackSegmentDirection::Left,
            TrackSegmentDirection::DownLeftDown,
            TrackSegmentDirection::DownRightRight,
            TrackSegmentDirection::Right,
            TrackSegmentDirection::Right,
            TrackSegmentDirection::Right,
            TrackSegmentDirection::Right,
            TrackSegmentDirection::Right,
            TrackSegmentDirection::Right,
            TrackSegmentDirection::DownRightDown,
            TrackSegmentDirection::DownLeftLeft,
            TrackSegmentDirection::Left,
            TrackSegmentDirection::Left,
            TrackSegmentDirection::Left,
            TrackSegmentDirection::Left,
            TrackSegmentDirection::Left,
            TrackSegmentDirection::Left,
            TrackSegmentDirection::Left,
            TrackSegmentDirection::UpLeftUp,
            TrackSegmentDirection::Up,
            TrackSegmentDirection::Up,
            TrackSegmentDirection::Up,
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

        if let Some(ref auto_drive) = self.auto_drive {
            let mut auto_drive = auto_drive.borrow_mut();
            auto_drive.tick(delta);
            auto_drive.evaluate(self.find::<car::Car>());
        }
    }
}
