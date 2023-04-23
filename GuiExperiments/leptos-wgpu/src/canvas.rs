use std::ops::Deref;

use leptos::html::{Canvas, Div};
use leptos::*;
use stylers::style;
use web_sys::HtmlCanvasElement;

use crate::resize_observer::{use_resize_observer, ResizeEventMode};
use crate::wgpu_render::render_triangle;

pub async fn async_test_func() {
    log!("In async test func");
}

#[component]
pub fn CanvasWrapper(cx: Scope) -> impl IntoView {
    let canvas: NodeRef<Canvas> = create_node_ref(cx);

    let on_mount = use_resize_observer::<Div, _>(cx, ResizeEventMode::BorderBoxSize, move |ev| {
        log!("Resized to: {:?}", ev);
        if let Some(canvas) = canvas.get() {
            canvas.set_width(ev.inline_size as u32);
            canvas.set_height(ev.block_size as u32);
            // TODO:
            // let style = canvas.style();
            // https://github.com/rustwasm/wasm-bindgen/issues/1334
            // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlCanvasElement.html#method.style
        }
    });

    let render = move |_| {
        let canvas = canvas().expect("<canvas> to exist");
        // let canvas: &HtmlCanvasElement = canvas.deref();
        // log!("{:?} {} {}", canvas, canvas.width(), canvas.height());
        // log!("Calling render_triangle...");
        // // spawn_local(asnyc_test_func());
        spawn_local(async move {
            let canvas: &HtmlCanvasElement = canvas.deref();
            log!("{:?} {} {}", canvas, canvas.width(), canvas.height());
            async_test_func().await;
            render_triangle(&canvas).await;
        });
    };

    /*
    let wrapper = view! {
        cx,
        class = STYLE,
            <div>
                <canvas node_ref=canvas />
            </div>
    }
    .on_mount(on_mount);
    */

    /*
    view! { cx,
        <div>
            <canvas node_ref=canvas />
        </div>
    }
    .on_mount(on_mount);
    */

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
        border: 1px solid darkblue;
        width: 100%;
        height: 600px;
    }

    canvas {
        width: 800px;
        height: 600px;
    }
};
