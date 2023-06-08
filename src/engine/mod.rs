use wasm_bindgen::closure::*;
use std::{rc::Rc, cell::*};
use web_sys::*;
use crate::browser::*;
use crate::game::*;
use wasm_bindgen::{ JsValue, JsCast };
mod input;
mod bounding_box;
mod vec;
mod renderer;
pub use input::*;
pub use bounding_box::*;
pub use vec::*;
pub use renderer::*;

#[derive(Default, Clone, Copy, Debug)]
pub struct Line {
    pub start: FVec,
    pub end: FVec
}

impl Line {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Line { start: FVec{x: x1, y: y1}, end: FVec{x: x2, y: y2} }
    }

    pub fn projection(&self, point: &FVec) -> Option<FVec> {
        let d = self.end.clone() - self.start.clone();
        let pq = point.clone() - self.start.clone();
        let t = (pq * d.clone()) / (d.clone() * d.clone());
        if t < 0.0 || t > 1.0 {
            return None;
        }
        let proj = d * t;
        Some(proj)
    }
        
    pub fn distance(&self, other: &Line) -> Option<f64> {
        if let Some(intersect) = self.intersect(other) {
            return Some(self.start.distance(&intersect));
        }
        None
    }

    pub fn intersect(&self, other: &Line) -> Option<FVec> {
        let x1 = self.start.x;
        let y1 = self.start.y;
        let x2 = self.end.x;
        let y2 = self.end.y;
        let x3 = other.start.x;
        let y3 = other.start.y;
        let x4 = other.end.x;
        let y4 = other.end.y;
        let ua = ((x4-x3)*(y1-y3) - (y4-y3)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));
        let ub = ((x2-x1)*(y1-y3) - (y2-y1)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));
        if ua >= 0.0 && ua <= 1.0 && ub >= 0.0 && ub <= 1.0 {
            let ix = x1 + (ua * (x2-x1));
            let iy = y1 + (ua * (y2-y1));
            return Some(FVec { x: ix, y: iy })
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

#[derive(Debug, Clone)]
pub struct Cell {
    frame: Rect,
    offset_x: f64,
    offset_y: f64,
    scale: f64,
    rotate: f64,
}

impl Cell {
    fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Cell { frame: Rect { x, y, w, h }, offset_x: 0.0, offset_y: 0.0, scale: 0.0, rotate: 0.0 }
    }
}

type SharedLoopClosure = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

pub struct RenderLoop {
    last_frame: f64,
    accumulated_delta: f64,
}

const FRAME_RATE: f64 = 1.0 / 60.0 * 1000.0;

impl RenderLoop {
    pub fn new() -> Self {
        RenderLoop {
            last_frame: 0.0,
            accumulated_delta: 0.0,
        }
    }

    pub fn start<T: Game + 'static>(mut self, mut game: T, canvas: HtmlCanvasElement) -> Result<(), JsValue> {
        let context = get_context(&canvas);
        if let Some(context) = context {
            let renderer = Renderer{ context };
            self.last_frame = now();
            let f: SharedLoopClosure = Rc::new(RefCell::new(None));
            let g = f.clone();
            *g.borrow_mut() = Some(Closure::wrap(Box::new(move |perf: f64| {
                let mut delta = (perf - self.last_frame);
                self.accumulated_delta += delta;
                while self.accumulated_delta > FRAME_RATE {
                    game.update(delta / 1000.0);
                    delta = self.accumulated_delta - FRAME_RATE;
                    self.accumulated_delta -= FRAME_RATE;
                }
                self.last_frame = perf;
                game.draw(&renderer);
                request_animation_frame(f.borrow().as_ref().unwrap());
            })));
            request_animation_frame(g.borrow().as_ref().unwrap())?;
            return Ok(());
        } else {
            return Err(JsValue::from_str("there is not context"));
        }
    }
}

fn get_context(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
    canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().ok()
}
