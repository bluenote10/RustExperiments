mod utils;

use crate::utils::console_log;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, Request, RequestInit, RequestMode, Response};

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub async fn load_data() -> Result<JsValue, JsValue> {
    console_log!("loading data...");

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    // let url = format!("http://127.0.0.1:3000/");
    // let url = format!("http://127.0.0.1:3000/plain_string");
    let url = format!("http://127.0.0.1:3000/binary_route");

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    console::log_1(&resp);

    let blob = JsFuture::from(resp.blob()?).await?;
    // let array_buffer = JsFuture::from(resp.array_buffer()?).await?;
    // let text = JsFuture::from(resp.text()?).await?;

    Ok(blob.into())
}

// Original example: https://github.com/rustwasm/wasm-bindgen/blob/main/examples/fetch/src/lib.rs

#[wasm_bindgen]
pub async fn demo_get() -> Result<JsValue, JsValue> {
    console_log!("loading data...");

    let repo: String = "rustwasm/wasm-bindgen".into();

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("https://api.github.com/repos/{}/branches/master", repo);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}
