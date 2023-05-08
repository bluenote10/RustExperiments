use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use leptos::ev::EventDescriptor;
use leptos::html::{Canvas, Div};
use leptos::*;
use stylers::style;
use web_sys::{HtmlCanvasElement, MouseEvent};

use crate::resize_observer::{use_resize_observer, ResizeEventMode};
use crate::wgpu_render::{render_msaa_line, Renderer};

pub async fn async_test_func() {
    log!("In async test func");
}

/*
fn take_f<F>(f: F)
where
    F: FnMut() + 'static,
{
}
*/

// fn take_f(mut f: impl FnMut() + 'static) {}

// fn take_f<E: EventDescriptor + 'static>(event: E, mut f: impl FnMut(E::EventType) + 'static) {}

fn take_f(mut f: impl FnMut() + 'static) {}

// https://github.com/leptos-rs/leptos/issues/1018
#[component]
pub fn CanvasWrapper2(cx: Scope) -> impl IntoView {
    let canvas: NodeRef<Canvas> = create_node_ref(cx);

    let renderer: Rc<RefCell<Option<Renderer>>> = Default::default();
    let renderer_clone = renderer.clone();

    let on_mount = move |parent: HtmlElement<Div>| {
        // [...] somewhere initialize the renderer instance.
        if let Some(canvas) = canvas.get() {
            let canvas: &HtmlCanvasElement = canvas.deref();
            *renderer_clone.borrow_mut() = Some(Renderer::new(canvas.clone()));
        }
    };

    let renderer_clone = renderer.clone();
    let cb = move || {
        let renderer = renderer_clone.clone();
        spawn_local(async move {
            let renderer = renderer.borrow();
            if let Some(ref renderer) = *renderer {
                Some(renderer.render())
            } else {
                None
            };
        });
    };
    //take_f(::leptos::ev::click, cb);
    take_f(cb);

    let renderer_clone = renderer.clone();
    let on_click = move |_| {
        let renderer = renderer_clone.clone();
        spawn_local(async move {
            let renderer = renderer.borrow();
            if let Some(ref renderer) = *renderer {
                Some(renderer.render())
            } else {
                None
            };
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

/*
#[allow(non_snake_case, clippy::too_many_arguments)]
pub fn CanvasWrapper3(
    #[allow(unused_variables)] cx: ::leptos::Scope,
    props: CanvasWrapper2Props,
) -> impl IntoView {
    pub fn __CanvasWrapper2(cx: Scope) -> impl IntoView {
        let canvas: NodeRef<Canvas> = create_node_ref(cx);
        let renderer: Rc<RefCell<Option<Renderer>>> = Default::default();
        let renderer_clone = renderer.clone();
        let on_mount = move |parent: HtmlElement<Div>| {
            if let Some(canvas) = canvas.get() {
                let canvas: &HtmlCanvasElement = canvas.deref();
                *renderer_clone.borrow_mut() = Some(Renderer::new(canvas.clone()));
            }
        };

        let renderer_clone = renderer.clone();
        let cb = move |_| {
            let renderer = renderer_clone;
            spawn_local(async move {
                let renderer = renderer.borrow();
                if let Some(renderer) = *renderer {
                    Some(renderer.render())
                } else {
                    None
                };
            });
        };
        take_f(::leptos::ev::click, cb);
        take_f(::leptos::ev::click, cb);

        let renderer_clone = renderer.clone();
        let on_click = |_| {
            let renderer = renderer_clone;
            spawn_local(async move {
                let renderer = renderer.borrow();
                if let Some(renderer) = *renderer {
                    Some(renderer.render())
                } else {
                    None
                };
            });
        };
        {
            leptos::Fragment::lazy(|| {
                <[_]>::into_vec(Box::new([
                    {
                        leptos::leptos_dom::html::div(cx)
                            .child((cx, leptos::leptos_dom::html::canvas(cx).node_ref(canvas)))
                            .with_view_marker("src-canvas.rs-68")
                            .on_mount(on_mount)
                    }
                    .into_view(cx),
                    leptos::leptos_dom::html::button(cx)
                        .on(::leptos::ev::click, on_click)
                        .child((
                            cx,
                            #[allow(unused_braces)]
                            "Render",
                        ))
                        .into_view(cx),
                ]))
            })
            .with_view_marker("src-canvas.rs-65")
        }
    }
    let CanvasWrapper2Props {} = props;
    ::leptos::leptos_dom::Component::new("CanvasWrapper2", move |cx| __CanvasWrapper2(cx))
}
*/

#[component]
pub fn CanvasWrapper(cx: Scope) -> impl IntoView {
    let canvas: NodeRef<Canvas> = create_node_ref(cx);

    let renderer: Rc<RefCell<Option<Renderer>>> = Default::default();

    let renderer_clone = renderer.clone();
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
            *renderer_clone.borrow_mut() = Some(Renderer::new(canvas.clone()));
        }
    });

    let renderer_clone = renderer.clone();
    let render = move |_| {
        let canvas = canvas().expect("<canvas> to exist");
        // let canvas: &HtmlCanvasElement = canvas.deref();
        // log!("{:?} {} {}", canvas, canvas.width(), canvas.height());
        // log!("Calling render_triangle...");
        // // spawn_local(async_test_func());
        /*
        spawn_local(async move {
            let canvas: &HtmlCanvasElement = canvas.deref();
            log!("{:?} {} {}", canvas, canvas.width(), canvas.height());
            async_test_func().await;
            render_msaa_line(canvas).await;
        });
        */

        let renderer = renderer_clone.clone();
        spawn_local(async move {
            let renderer = renderer.borrow();
            if let Some(ref renderer) = *renderer {
                Some(renderer.render().await)
            } else {
                None
            };
        });
        /*
        spawn_local(async move {
            let renderer = renderer.clone();
            let opt_future = renderer.with_value(async |renderer| {
                if let Some(renderer) = renderer {
                    Some(renderer.render().await)
                } else {
                    None
                }
            });
            if let Some(future) = opt_future {
                future.await;
            }
        });
        */

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
