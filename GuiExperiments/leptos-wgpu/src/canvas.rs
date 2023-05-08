use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use leptos::html::{Canvas, Div};
use leptos::*;
use stylers::style;
use web_sys::HtmlCanvasElement;

use crate::resize_observer::{use_resize_observer, ResizeEventMode};
use crate::wgpu_render::Renderer;

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

            let canvas: &HtmlCanvasElement = canvas.deref();
            *renderer.borrow_mut() = Some(Renderer::new(canvas.clone()));
        }
    });

    let renderer = renderer_orig.clone();
    let render = move |_| {
        let renderer = renderer.clone();
        spawn_local(async move {
            let renderer = renderer.borrow();
            if let Some(ref renderer) = *renderer {
                Some(renderer.render().await)
            } else {
                None
            };
        });
    };

    view! {
        cx,
        class = STYLE,
        <>
            {
                view! { cx, class = STYLE,
                    <div>
                        <canvas node_ref=canvas />
                    </div>
                }.on_mount(on_mount)
            }
            <button on:click=render>"Render"</button>
        </>
    }
}

const STYLE: &str = style! {"Canvas",
    div {
        margin: 10px;
        height: 600px;
    }

    canvas {
        width: 800px;
        height: 600px;
        border: 1px solid darkblue;
        background-color: #FAFAFA;
    }
};
