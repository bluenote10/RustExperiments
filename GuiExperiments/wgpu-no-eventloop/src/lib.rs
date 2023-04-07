mod utils;
mod wgpu_render;

use crate::{utils::console_log, wgpu_render::render_triangle};

use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlCanvasElement};

fn create_canvas_element(document: &Document) -> Result<HtmlCanvasElement, JsValue> {
    let canvas = document.create_element("canvas")?;
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    Ok(canvas)
}

fn check_canvas_context(canvas: &HtmlCanvasElement) -> String {
    if canvas.get_context("2d").unwrap().is_some() {
        "2d".into()
    } else if canvas.get_context("webgl").unwrap().is_some() {
        "webgl".into()
    } else if canvas.get_context("webgl2").unwrap().is_some() {
        "webgl2".into()
    } else if canvas.get_context("webgpu").unwrap().is_some() {
        "webgpu".into()
    } else if canvas.get_context("bitmaprenderer").unwrap().is_some() {
        "bitmaprenderer".into()
    } else {
        "unknown".into()
    }
}

async fn add_canvas() -> Result<(), JsValue> {
    // Inspired by:
    // https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let canvas = create_canvas_element(&document)?;
    canvas.set_width(800);
    canvas.set_height(600);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let root_div = document.get_element_by_id("root").unwrap();
    root_div.append_child(&canvas)?;

    context.set_font("20px monospace");
    context.fill_text("Hello world", 100.0, 100.0)?;
    console_log!("Type of canvas: {}", check_canvas_context(&canvas));

    // WebGL canvas
    let canvas = create_canvas_element(&document)?;
    canvas.set_width(800);
    canvas.set_height(600);
    root_div.append_child(&canvas)?;

    // Apparently we are not allowed to construct a context ourselves, otherwise
    // wgpu will fail to construct a surface out of it.
    // let webgl_context = canvas.get_context("webgl")?;
    // console_log!("webgl_context: {:?}", webgl_context);

    render_triangle(&canvas).await;
    console_log!("Type of canvas: {}", check_canvas_context(&canvas));

    // WebGPU canvas
    let canvas = create_canvas_element(&document)?;
    canvas.set_width(800);
    canvas.set_height(600);
    root_div.append_child(&canvas)?;

    let webgpu_context = canvas.get_context("webgpu")?;
    console_log!("webgpu_context: {:?}", webgpu_context);
    console_log!("Type of canvas: {}", check_canvas_context(&canvas));

    Ok(())
}

#[wasm_bindgen(start)]
async fn main() -> Result<(), JsValue> {
    console_log!("Initializing WASM...");
    console_error_panic_hook::set_once();

    add_canvas().await?;

    Ok(())
}
