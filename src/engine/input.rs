use std::collections::*;
use futures::channel::mpsc::*;
use web_sys::*;
use crate::browser::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct KeyboardState {
    pressed_keys: HashMap<String, KeyboardEvent>,
}

impl KeyboardState {
    pub fn new() -> Self {
        KeyboardState {
            pressed_keys: HashMap::new()
        }
    }

    pub fn press(&mut self, event: KeyboardEvent) {
        self.pressed_keys.insert(event.code(), event);
    }

    pub fn is_pressed(&self, code: &str) -> bool {
        self.pressed_keys.contains_key(code)
    }

    pub fn set_released(&mut self, code: &str) {
        self.pressed_keys.remove(code);
    }
}

pub fn process_input(receiver: &mut UnboundedReceiver<KeyPress>, state: Rc<RefCell<KeyboardState>>) {
    loop {
        match receiver.try_next() {
            Ok(None) => {
                break
            },
            Err(err) => {
                break
            },
            Ok(Some(event)) => match event {
                KeyPress::KeyDown(event) => {
                    state.borrow_mut().press(event);
                },
                KeyPress::KeyUp(event) => {
                    state.borrow_mut().set_released(event.code().as_str())
                },
            }
        }
    }
}
