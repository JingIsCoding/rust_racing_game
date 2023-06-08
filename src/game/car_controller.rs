use std::{rc::Rc, cell::RefCell};
use super::car::*;
use super::*;

pub trait CarController {
    fn next_movements(&self, car: &Car) -> Vec<Movement>;
}

pub struct KeyController {
    keyboard_state: Rc<RefCell<KeyboardState>>
}

impl KeyController {
    pub fn new(keyboard_state: Rc<RefCell<KeyboardState>>) -> Self {
        KeyController { keyboard_state }
    }
}

pub struct SimpleController {
}

impl CarController for SimpleController {
    fn next_movements(&self, car: &Car) -> Vec<Movement> {
        let mut movements = vec![];
        movements.push(Movement::Forward);
        return movements;
    }
}

impl CarController for KeyController {
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
