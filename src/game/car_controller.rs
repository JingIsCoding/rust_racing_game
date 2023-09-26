use std::{rc::Rc, cell::RefCell, collections::HashMap, f64::consts::PI};
use super::car::*;
use super::*;
use crate::engine::network::post;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::fmt::Debug as FmtDebug;

pub trait CarController: FmtDebug {
    fn get_id(&self) -> Uuid;
    fn next_movements(&self, car: &Car) -> Vec<Movement>;
}

#[derive(Debug)]
pub struct KeyController {
    id: Uuid,
    keyboard_state: Rc<RefCell<KeyboardState>>
}

impl KeyController {
    pub fn new(id: Uuid, keyboard_state: Rc<RefCell<KeyboardState>>) -> Self {
        KeyController { 
            id,
            keyboard_state 
        }
    }
}


#[derive(Debug)]
pub struct SimpleController {
}

#[derive(Debug)]
struct Timer {
    timeout: f64,
    ellapsed: f64
}

impl Timer {
    fn new(timeout: f64) -> Self {
        Timer {
            timeout,
            ellapsed: 0.0
        }
    }

    fn tick(&mut self, delta: f64) {
        self.ellapsed += delta;
    }

    fn reset(&mut self) {
        self.ellapsed = self.ellapsed % self.timeout;
    }

    fn timeout(&self) -> bool {
        return self.ellapsed > self.timeout
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NewGenRequest {
    num_of_cars: u32,
    num_of_args: u32,
    num_of_outputs: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluteRequest {
    pub inputs: Vec<Vec<f64>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetFitnessesRequest {
    pub fitnesses: Vec<f64>
}

#[derive(Debug)]
pub struct AutoDrive {
    controllers: Rc<RefCell<HashMap<Uuid, Vec<f64>>>>,
    timer: Timer
}

impl AutoDrive {
    pub fn new(update_interval: f64) -> Self {
        AutoDrive { 
            timer: Timer::new(update_interval),
            controllers: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn register(&mut self, id: Uuid) {
        self.controllers.borrow_mut().insert(id, vec![]);
    }

    pub async fn new_network(&mut self) {
        let request = NewGenRequest{
            num_of_cars: self.controllers.borrow().len() as u32,
            num_of_args: 7,
            num_of_outputs: 4,
        };
        let request = serde_json::to_string(&request).unwrap();
        let request = Some(request.as_str());
        let result = post("http://127.0.0.1:3030/api/gen_network", request).await;
        if let Err(value) = result {
            crate::log(value.as_string().unwrap().as_str());
        }
    }

    pub fn evaluate(&mut self, cars: Vec<&car::Car>) {
        if !self.timer.timeout() {
            return
        }
        self.timer.reset();
        let controllers = self.controllers.clone();
        let ids: Vec<Uuid> = cars.iter().map(| car | car.id).collect();
        let request = EvaluteRequest{
            inputs: cars.iter().map(|car| { 
                let forward = car.sensor.forward_dis / car_sensor::SENSOR_RANGE;
                let right = car.sensor.right_dis / car_sensor::SENSOR_RANGE;
                let back = car.sensor.back_dis / car_sensor::SENSOR_RANGE;
                let left = car.sensor.left_dis / car_sensor::SENSOR_RANGE;
                let velocity = car.velocity / car_sensor::SENSOR_RANGE;
                let rotate = car.body.rotate / PI;
                let track_dir = car.sensor.track_direction / PI;
                vec![forward, right, back, left, velocity, rotate, track_dir] 
            }).collect(),
        };
        spawn_local(async move {
            let request = serde_json::to_string(&request).unwrap();
            let request = Some(request.as_str());
            let result = post("http://127.0.0.1:3030/api/evaluate_network", request).await;
            if let Ok(ref result) = result {
                if let Some(result) = result.as_string() {
                    if let Ok(outputs) = serde_json::from_str::<Vec<Vec<f64>>>(&result) {
                        let mut controllers = controllers.borrow_mut();
                        for (index, id) in ids.iter().enumerate() {
                            controllers.insert(*id, outputs[index].clone());
                        }
                    }
                }
            }
        });
    }

    pub fn next_gen(&mut self, cars: Vec<&car::Car>) {
        let scores: Vec<f64> = cars.iter().map(| car | {
            car.score.score
        }).collect();
        crate::console_log!("max fitness is {:?}", scores.iter().max_by(| x, y | { 
            if x > y {
                return std::cmp::Ordering::Greater;
            }
            return std::cmp::Ordering::Less;
        }));
        spawn_local(async move {
            let request = SetFitnessesRequest{
                fitnesses: scores
            };
            let request = serde_json::to_string(&request).unwrap();
            let request = Some(request.as_str());
            let result = post("http://127.0.0.1:3030/api/set_fitness", request).await;
            if let Err(value) = result {
                crate::log(value.as_string().unwrap().as_str());
                return
            }

            let result = post("http://127.0.0.1:3030/api/next_gen", None).await;
            if let Err(value) = result {
                crate::log(value.as_string().unwrap().as_str());
                return
            }
        });
    }

    pub fn tick(&mut self, delta: f64) {
        self.timer.tick(delta)
    }
}

#[derive(Debug)]
pub struct AutoDriveController {
    id: Uuid,
    auto_drive: Rc<RefCell<AutoDrive>>
}

impl AutoDriveController {
    pub fn new(id: Uuid, auto_drive: Rc<RefCell<AutoDrive>>) -> Self {
        auto_drive.borrow_mut().register(id);
        let controller = AutoDriveController {
            id,
            auto_drive,
        };
        controller
    }
}

impl CarController for AutoDriveController {
    fn get_id(&self) -> Uuid {
        return self.id
    }

    fn next_movements(&self, _car: &Car) -> Vec<Movement> {
        let mut movements = vec![];
        if let Some(outputs) = self.auto_drive.borrow().controllers.borrow().get(&self.id) {
            for (i, output) in outputs.iter().enumerate() {
                match i {
                    0 => {
                        if *output > 0.5 {
                            movements.push(Movement::Forward);
                        }
                    },
                    1 => {
                        if *output > 0.5 {
                            movements.push(Movement::Right);
                        }
                    },
                    2 => {
                        if *output > 0.5 {
                            movements.push(Movement::Backward);
                        }
                    },
                    3 => {
                        if *output > 0.5 {
                            movements.push(Movement::Left);
                        }
                    },
                    _ => {}
                }
            }
        }
        return movements;
    }
}

impl CarController for SimpleController {
    fn get_id(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn next_movements(&self, car: &Car) -> Vec<Movement> {
        let mut movements = vec![];
        movements.push(Movement::Forward);
        return movements;
    }
}

impl CarController for KeyController {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn next_movements(&self, car: &Car) -> Vec<Movement> {
        let keyboard_state = self.keyboard_state.clone();
        let mut movements = vec![];
        if keyboard_state.borrow().is_pressed("ArrowLeft") {
            movements.push(Movement::Left);
        } else if keyboard_state.borrow().is_pressed("ArrowRight") {
            movements.push(Movement::Right);
        } else {
            movements.push(Movement::NotTurning);
        }
        if keyboard_state.borrow().is_pressed("ArrowUp") {
            movements.push(Movement::Forward);
        } else if keyboard_state.borrow().is_pressed("ArrowDown") {
            movements.push(Movement::Backward);
        } else {
            movements.push(Movement::NotAccelarate);
        }
        return movements;
    }
}
