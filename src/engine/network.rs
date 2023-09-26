use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture; // Add this line
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response;
use web_sys::Headers;

pub async fn post(url: &str, data: Option<&str>) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    let headers = Headers::new()?;
    headers.set("Content-Type", "application/json")?;
    opts.headers(&headers);
    if let Some(data) = data {
        opts.body(Some(&JsValue::from(data)));
    }
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().expect("no global `window` exists");
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp_value.dyn_into()?;
    let text = JsFuture::from(resp.text()?).await?;

    Ok(text)
}
