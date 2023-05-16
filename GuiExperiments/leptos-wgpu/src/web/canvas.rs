use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use leptos::html::{Canvas, Div};
use leptos::*;
use stylers::style;
use web_sys::{HtmlCanvasElement, MouseEvent};

use crate::web::resize_observer::{use_resize_observer, ResizeEventMode};
use crate::web::wgpu_render::Renderer;

#[component]
pub fn CanvasWrapper(cx: Scope) -> impl IntoView {
    let canvas: NodeRef<Canvas> = create_node_ref(cx);

    let renderer_orig: Rc<RefCell<Option<Renderer>>> = Default::default();

    let renderer = renderer_orig.clone();
    let on_mount = use_resize_observer::<Div, _>(cx, ResizeEventMode::BorderBoxSize, move |ev| {
        log!("Resized to: {:?}", ev);
        if let Some(canvas) = canvas.get() {
            canvas.set_width(ev.inline_size as u32);
            canvas.set_height(ev.block_size as u32);
            // Setting style requires:
            // https://github.com/rustwasm/wasm-bindgen/issues/1334
            // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlCanvasElement.html#method.style
            let style = canvas.deref().deref().style();
            style
                .set_property("width", &format!("{}px", ev.inline_size as u32,))
                .expect("Failed to set canvas size");
            style
                .set_property("height", &format!("{}px", ev.block_size as u32,))
                .expect("Failed to set canvas size");

            let renderer = renderer.clone();
            spawn_local(async move {
                let canvas: &HtmlCanvasElement = canvas.deref();
                *renderer.borrow_mut() = Some(Renderer::new(canvas.clone()).await);
            });
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

    let renderer = renderer_orig.clone();
    let on_mousemove = move |event: MouseEvent| {
        let mut renderer = renderer.borrow_mut();
        if let Some(canvas) = canvas.get() {
            // https://stackoverflow.com/a/2614472/1804173
            let w = canvas.width();
            let h = canvas.height();
            let canvas: &HtmlCanvasElement = canvas.deref();
            let rect = canvas.get_bounding_client_rect();

            let x = event.client_x() - rect.x() as i32;
            let y = event.client_y() - rect.y() as i32;
            log!("{x} {y}");
            if let Some(ref mut renderer) = *renderer {
                renderer.set_transform(x + 30, y + 30, w, h);
                renderer.render();
            };
        }
    };

    view! {
        cx,
        class = STYLE,
        <>
            {
                view! { cx, class = STYLE,
                    <div>
                        <canvas node_ref=canvas on:mousemove=on_mousemove/>
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
