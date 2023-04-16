use std::ops::Deref;

use leptos::html::Canvas;
use leptos::*;
use stylers::style;
use web_sys::HtmlCanvasElement;

use crate::wgpu_render::render_triangle;

pub async fn asnyc_test_func() {
    log!("In async test func");
}

#[component]
pub fn CanvasWrapper(cx: Scope) -> impl IntoView {
    let canvas: NodeRef<Canvas> = create_node_ref(cx);

    let render = move |_| {
        let canvas = canvas().expect("<input> to exist");
        // let canvas: &HtmlCanvasElement = canvas.deref();
        // log!("{:?} {} {}", canvas, canvas.width(), canvas.height());
        // log!("Calling render_triangle...");
        // // spawn_local(asnyc_test_func());
        spawn_local(async move {
            let canvas: &HtmlCanvasElement = canvas.deref();
            log!("{:?} {} {}", canvas, canvas.width(), canvas.height());
            asnyc_test_func().await;
            render_triangle(&canvas).await;
        });
    };

    view! {
        cx,
        class = STYLE,
        <>
            <div>
                <canvas node_ref=canvas />
            </div>
            <button on:click=render>"Render"</button>
        </>
    }
}

const STYLE: &str = style! {"Canvas",
    div {
        border: 1px solid darkblue;
        width: 800px;
        height: 600px;
    }

    canvas {
        width: 800px;
        height: 600px;
    }
};
