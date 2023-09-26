use crate::{engine::*, browser::prepare_input};
use crate::browser::*;
use std::any::Any;
mod car;
mod car_sensor;
mod car_controller;
mod stage;
mod track;
mod score;
use stage::*;
use futures::channel::mpsc::*;
use std::rc::Rc;
use std::cell::RefCell;

pub trait GameObject: Any {
    fn draw(&self, renderer: &Renderer);
    fn update(&mut self, stage: &mut GameStage, delta: f64);
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

pub struct RacingGame {
    pub width: f64,
    pub height: f64,
    pub keyboard_state: Rc<RefCell<KeyboardState>>,
    receiver: UnboundedReceiver<KeyPress>,
    current_stage: Option<Box<GameStage>>,
}

impl RacingGame {
    pub async fn new(width: f64, height: f64) -> Self {
        let receiver = prepare_input().unwrap();
        let keyboard_state = Rc::new(RefCell::new(KeyboardState::new()));
        RacingGame {
            width,
            height,
            receiver,
            keyboard_state: keyboard_state.clone(),
            current_stage: Some(GameStage::new(keyboard_state.clone()).await),
        }
    }
}

impl RacingGame {
    pub fn draw(&mut self, renderer: &Renderer) {
        renderer.clear(&Rect{ x: 0.0, y:0.0, w: self.width, h: self.height });
        if let Some(stage) = &self.current_stage {
            stage.draw(renderer);
        }
    }

    pub fn update(&mut self, delta: f64) {
        process_input(&mut self.receiver, self.keyboard_state.clone());
        if let Some(ref mut stage) = self.current_stage {
            stage.update(delta);
        }
    }
}
