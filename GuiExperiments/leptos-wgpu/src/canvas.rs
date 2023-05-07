use std::ops::Deref;

use leptos::html::{Canvas, Div};
use leptos::*;
use stylers::style;
use web_sys::HtmlCanvasElement;

use crate::resize_observer::{use_resize_observer, ResizeEventMode};
use crate::wgpu_render::{render_msaa_line, Renderer};

pub async fn async_test_func() {
    log!("In async test func");
}

/*
// https://github.com/leptos-rs/leptos/issues/1018
#[component]
pub fn CanvasWrapper2(cx: Scope) -> impl IntoView {
    let canvas: NodeRef<Canvas> = create_node_ref(cx);

    let renderer: StoredValue<Option<Renderer>> = store_value(cx, None);

    let on_mount = move |parent: HtmlElement<Div>| {
        // [...] somewhere initialize the renderer instance.
        if let Some(canvas) = canvas.get() {
            let canvas: &HtmlCanvasElement = canvas.deref();
            renderer.set_value(Some(Renderer::new(canvas.clone())));
        }
    };

    let on_click = move |_| {
        spawn_local(async move {
            renderer.with_value(|renderer| {
                if let Some(renderer) = renderer {
                    Some(renderer.render())
                } else {
                    None
                }
            });
        });
    };

    view! { cx,
        <>
            {
                view! { cx,
                    <div>
                        <canvas node_ref=canvas />
                    </div>
                }.on_mount(on_mount)
            }
            <button on:click=on_click>"Render"</button>
        </>
    }
}
*/

#[component]
pub fn CanvasWrapper(cx: Scope) -> impl IntoView {
    let canvas: NodeRef<Canvas> = create_node_ref(cx);

    let renderer: StoredValue<Option<Renderer>> = store_value(cx, None);

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
            renderer.set_value(Some(Renderer::new(canvas.clone())));
        }
    });

    let render = move |_| {
        let canvas = canvas().expect("<canvas> to exist");
        // let canvas: &HtmlCanvasElement = canvas.deref();
        // log!("{:?} {} {}", canvas, canvas.width(), canvas.height());
        // log!("Calling render_triangle...");
        // // spawn_local(async_test_func());
        spawn_local(async move {
            let canvas: &HtmlCanvasElement = canvas.deref();
            log!("{:?} {} {}", canvas, canvas.width(), canvas.height());
            async_test_func().await;
            render_msaa_line(canvas).await;
        });

        /*
        let async_render = || async move {
            renderer.with_value(|renderer| {
                if let Some(renderer) = renderer {
                    Some(renderer.render())
                } else {
                    None
                }
            });
        };
        */
        /*
        spawn_local(async move {
            renderer.with_value(|renderer| {
                if let Some(renderer) = renderer {
                    Some(renderer.render())
                } else {
                    None
                }
            });
        });
        */
        /*
        renderer.with_value(|renderer| async move {
            if let Some(renderer) = renderer {
                renderer.render().await;
            }
        });
        */
        /*
        renderer.with_value(|renderer| {
            spawn_local(async {
                if let Some(renderer) = renderer {
                    renderer.render().await;
                }
            });
        });
        */
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
