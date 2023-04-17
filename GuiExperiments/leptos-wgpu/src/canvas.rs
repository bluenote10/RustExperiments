use std::ops::Deref;

use leptos::html::{Canvas, Div};
use leptos::*;
use stylers::style;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{
    HtmlCanvasElement, HtmlDivElement, ResizeObserver, ResizeObserverEntry, ResizeObserverSize,
};

use crate::wgpu_render::render_triangle;

pub async fn asnyc_test_func() {
    log!("In async test func");
}

#[component]
pub fn CanvasWrapper(cx: Scope) -> impl IntoView {
    let canvas: NodeRef<Canvas> = create_node_ref(cx);

    let resize_callback = Closure::<dyn Fn(Vec<ResizeObserverEntry>)>::new(
        move |entries: Vec<ResizeObserverEntry>| {
            log!("Resize callback. Num entries: {}", entries.len());
            for entry in entries {
                // Array due to possible fragments. Under normal circumstances just the element 0 matters.
                // https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/borderBoxSize
                log!("{:?}", entry.content_box_size().get(0));
                let content_box_size: ResizeObserverSize = entry.content_box_size().get(0).into();
                log!(
                    "{:?} {:?}",
                    content_box_size.block_size(),
                    content_box_size.inline_size()
                );
            }
        },
    );

    let resize_observer = ResizeObserver::new(resize_callback.as_ref().unchecked_ref())
        .expect("Failed to create resize observer");

    // Memory leak here:
    // https://rustwasm.github.io/wasm-bindgen/examples/closures.html
    // resize_callback.forget();

    let on_mount = move |el: HtmlElement<Div>| {
        log!("Mounting parent");
        let resize_observer = resize_observer.clone();
        let el: HtmlDivElement = el.deref().clone();
        resize_observer.observe(&el.into());
    };

    on_cleanup(cx, move || {
        log!("Cleaning up (1)");
        // This is needed to let the callback live long enough.
        // https://rustwasm.github.io/wasm-bindgen/examples/closures.html
        // https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html
        let _cleanup = resize_callback;
        //let resize_observer = resize_observer;
    });

    on_cleanup(cx, move || {
        log!("Cleaning up (2)");
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
            asnyc_test_func().await;
            render_triangle(&canvas).await;
        });
    };

    let wrapper = view! {
        cx,
        class = STYLE,
            <div>
                <canvas node_ref=canvas />
            </div>
    }
    .on_mount(on_mount);

    view! {
        cx,
        class = STYLE,
        <>
            {wrapper}
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
