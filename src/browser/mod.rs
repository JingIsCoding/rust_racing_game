use std::{rc::Rc, cell::RefCell};
use std::sync::Mutex;
use std::future::Future;
use web_sys::*;
use futures::channel::mpsc::*;
use wasm_bindgen::{JsCast, JsValue, closure, prelude::Closure};

pub fn window() -> Window {
    web_sys::window().expect("can not get window")
}

pub fn document() -> Document {
    window().document().expect("can not get document")
}

pub fn element<T: JsCast>(id: &str) -> Option<T> {
    document().get_element_by_id(id)?.dyn_into::<T>().ok()
}

pub fn now() -> f64 {
    window().performance().unwrap().now()
}

pub enum KeyPress {
    KeyDown(KeyboardEvent),
    KeyUp(KeyboardEvent)
}

pub fn prepare_input() -> Result<UnboundedReceiver<KeyPress>, JsValue> {
    let (keydown_sender, receiver) = unbounded();
    let keydown_sender = Rc::new(RefCell::new(keydown_sender));
    let keyup_sender = Rc::clone(&keydown_sender);
    let on_keydown: Closure<dyn FnMut(KeyboardEvent)> = wasm_bindgen::closure::Closure::new(move |event: KeyboardEvent| {
        keydown_sender.borrow_mut().start_send(KeyPress::KeyDown(event));
    });
    let on_keyup: Closure<dyn FnMut(KeyboardEvent)> = wasm_bindgen::closure::Closure::new(move |event: KeyboardEvent| {
        keyup_sender.borrow_mut().start_send(KeyPress::KeyUp(event));
    });
    let canvas = element::<HtmlCanvasElement>("game_canvas").unwrap();
    canvas.set_onkeydown(Some(on_keydown.as_ref().unchecked_ref()));
    canvas.set_onkeyup(Some(on_keyup.as_ref().unchecked_ref()));
    on_keydown.forget();
    on_keyup.forget();
    Ok(receiver)
}

pub fn spawn_local<F>(future: F) 
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

pub async fn load_image(src: &str) -> Result<HtmlImageElement, JsValue> {
    let new_image = web_sys::HtmlImageElement::new()?;
    let (success_tx, success_rs) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
    let success_tx = Rc::new(Mutex::new(Some(success_tx)));
    let error_tx = success_tx.clone();
    let success_cb = closure::Closure::once(move || {
        if let Some(sender) = success_tx.lock().ok().and_then(| mut sender | sender.take()) {
            sender.send(Ok(()));
        };
    });
    let error_cb = closure::Closure::once(move |err| {
        if let Some(sender) = error_tx.lock().ok().and_then(| mut sender | sender.take()) {
            sender.send(Err(err));
        };
    });
    new_image.set_onload(Some(success_cb.as_ref().unchecked_ref()));
    new_image.set_onerror(Some(error_cb.as_ref().unchecked_ref()));
    new_image.set_src(src);
    success_rs.await.map_err(|_| JsValue::from_str("can not get image"))??;
    Ok(new_image)
}

pub fn request_animation_frame(callback: &Closure<dyn FnMut(f64)>) -> Result<i32, JsValue> {
    window().request_animation_frame(callback.as_ref().unchecked_ref())
}
