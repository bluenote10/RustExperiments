use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use leptos::html::{Canvas, Div};
use leptos::*;
use stylers::style;
use web_sys::{HtmlCanvasElement, MouseEvent};

use crate::web::resize_observer::{use_resize_observer, ResizeEventMode};
use crate::web::wgpu_render::{Renderer, Viewport};

#[component]
pub fn CanvasWrapper(cx: Scope) -> impl IntoView {
    let canvas: NodeRef<Canvas> = create_node_ref(cx);

    let renderer_orig: Rc<RefCell<Option<Renderer>>> = Default::default();

    let viewport = store_value(
        cx,
        Viewport {
            x_from: 0.0,
            x_upto: 1.0,
            y_from: 0.0,
            y_upto: 1.0,
        },
    );

    let renderer = renderer_orig.clone();
    let on_mount = use_resize_observer::<Div, _>(cx, ResizeEventMode::BorderBoxSize, move |ev| {
        log!("Resized to: {:?}", ev);
        if let Some(canvas) = canvas.get() {
            let w = ev.inline_size as u32;
            let h = ev.block_size as u32;
            canvas.set_width(w);
            canvas.set_height(h);
            // Setting style requires:
            // https://github.com/rustwasm/wasm-bindgen/issues/1334
            // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlCanvasElement.html#method.style
            let style = canvas.deref().deref().style();
            style
                .set_property("width", &format!("{}px", w))
                .expect("Failed to set canvas size");
            style
                .set_property("height", &format!("{}px", h))
                .expect("Failed to set canvas size");

            let renderer = renderer.clone();
            spawn_local(async move {
                let canvas: &HtmlCanvasElement = canvas.deref();
                *renderer.borrow_mut() = Some(Renderer::new(canvas.clone()).await);
            });
            // Store width, height, and client bounding box in a stored value for later access?
            viewport.set_value(Viewport {
                x_from: 0.0,
                x_upto: w as f32,
                y_from: 0.0,
                y_upto: h as f32,
            });
            log!("Set viewport to: {:?}", viewport.get_value());
        }
    });

    let renderer = renderer_orig.clone();
    let render = move |_| {
        let mut renderer = renderer.borrow_mut();
        if let Some(ref mut renderer) = *renderer {
            renderer.set_render_data();
            renderer.render();
        };
    };

    let last_drag_mouse_coordinate: StoredValue<Option<(i32, i32)>> = store_value(cx, None);

    let renderer = renderer_orig.clone();
    let on_mousemove = move |event: MouseEvent| {
        let mut renderer = renderer.borrow_mut();
        if let Some(canvas) = canvas.get() {
            if let Some((last_x, last_y)) = last_drag_mouse_coordinate.get_value() {
                let x = event.client_x();
                let y = event.client_y();
                let dx = x - last_x;
                let dy = y - last_y;
                viewport.update_value(|viewport| viewport.translate(-dx as f32, -dy as f32));
                last_drag_mouse_coordinate.set_value(Some((x, y)));
            }
            // https://stackoverflow.com/a/2614472/1804173
            let canvas: &HtmlCanvasElement = canvas.deref();
            let rect = canvas.get_bounding_client_rect();

            let x = event.client_x() - rect.left() as i32;
            let y = event.client_y() - rect.top() as i32;
            log!("{x} {y}");
            if let Some(ref mut renderer) = *renderer {
                renderer.set_viewport(viewport.get_value());
                renderer.render();
            };
        }
    };

    let on_mousedown = move |event: MouseEvent| {
        let x = event.client_x();
        let y = event.client_y();
        last_drag_mouse_coordinate.set_value(Some((x, y)));
    };

    let on_mouseup = move |_: MouseEvent| {
        last_drag_mouse_coordinate.set_value(None);
    };

    view! {
        cx,
        class = STYLE,
        <>
            {
                view! { cx, class = STYLE,
                    <div>
                        <canvas node_ref=canvas on:mousemove=on_mousemove on:mousedown=on_mousedown on:mouseup=on_mouseup/>
                    </div>
                }.on_mount(on_mount)
            }
            <button on:click=render>"Render"</button>
        </>
    }
}

const STYLE: &str = style! {"Canvas",
    div {
        margin: 20px;
        height: 600px;
    }

    canvas {
        width: 800px;
        height: 600px;
        border: 1px solid darkblue;
        background-color: #FAFAFA;
    }
};
