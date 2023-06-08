use browser::element;
use engine::RenderLoop;
use game::RacingGame;
use wasm_bindgen::prelude::*;
use web_sys::{ HtmlCanvasElement, console };
use wasm_bindgen::{JsCast, JsValue, closure, prelude::Closure};
mod browser;
mod engine;
mod game;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    // Your code goes here!
    console::log_1(&JsValue::from_str("start game!"));

    let game_loop = RenderLoop::new();
    let canvas = element::<HtmlCanvasElement>("game_canvas").ok_or(JsValue::from_str("did not find canvas"))?;
    let racing_game = RacingGame::new(canvas.width() as f64, canvas.height() as f64).await;
    game_loop.start(racing_game, canvas)?;
    Ok(())
}
