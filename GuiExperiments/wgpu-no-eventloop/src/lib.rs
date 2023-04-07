mod utils;

use crate::utils::console_log;

use std::f64;
use wasm_bindgen::prelude::*;

fn add_canvas() -> Result<(), JsValue> {
    // Inspired by:
    // https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let canvas = document.create_element("canvas")?;
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let root_div = document.get_element_by_id("root").unwrap();
    root_div.append_child(&canvas)?;

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();

    Ok(())
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    console_log!("Initializing WASM...");
    console_error_panic_hook::set_once();
    add_canvas().unwrap();
    Ok(())
}
