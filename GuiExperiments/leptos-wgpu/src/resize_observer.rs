use std::ops::Deref;

use js_sys::Array;
use leptos::html::ElementDescriptor;
use leptos::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{ResizeObserver, ResizeObserverEntry, ResizeObserverSize};

// Loosely based on:
// https://github.com/sveltejs/svelte/pull/8022/files
//
// Note that it would probably be better to use a singleton for performance reasons:
// https://groups.google.com/a/chromium.org/g/blink-dev/c/z6ienONUb5A/m/F5-VcUZtBAAJ

#[derive(Copy, Clone, Debug)]
pub struct ResizeEvent {
    pub block_size: f64,
    pub inline_size: f64,
}

#[derive(Copy, Clone, Debug)]
pub enum ResizeEventMode {
    BorderBoxSize,
    ContentBoxSize,
    DevicePixelContentBoxSize,
}

pub fn use_resize_observer<El, F>(
    cx: Scope,
    mode: ResizeEventMode,
    f: F,
) -> impl Fn(HtmlElement<El>)
where
    El: ElementDescriptor + Deref,
    <El as Deref>::Target: Sized + Clone + Into<web_sys::Element>,
    F: Fn(ResizeEvent) + Clone + 'static,
{
    let resize_callback = Closure::<dyn Fn(Vec<ResizeObserverEntry>)>::new(
        move |entries: Vec<ResizeObserverEntry>| {
            for entry in entries {
                // Note that in general the box sizes are encoded as an array to support elements that have
                // multiple fragments, which occur in multi-column scenarios. For now I've simplified the API
                // here and simply return the first element, which is sufficient for normal resize usages.
                // See: https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/borderBoxSize
                let size = match mode {
                    ResizeEventMode::BorderBoxSize => {
                        try_extract_resize_observer_size(entry.border_box_size())
                    }
                    ResizeEventMode::ContentBoxSize => {
                        try_extract_resize_observer_size(entry.content_box_size())
                    }
                    ResizeEventMode::DevicePixelContentBoxSize => {
                        try_extract_resize_observer_size(entry.device_pixel_content_box_size())
                    }
                };
                if let Some(size) = size {
                    f(size);
                };
            }
        },
    );

    let resize_observer = ResizeObserver::new(resize_callback.as_ref().unchecked_ref())
        .expect("Failed to create resize observer");

    let on_mount = move |el: HtmlElement<El>| {
        log!("Registering resize observer");
        let resize_observer = resize_observer.clone();
        let el: <El as std::ops::Deref>::Target = el.deref().clone();
        resize_observer.observe(&el.into());

        // Here I'd something like the following, but that doesn't work due to lifetimes.
        // on_cleanup(cx, move || {
        //     log!("Cleaning up resize observer");
        //     resize_observer.unobserve(&el.into());
        // });
    };

    on_cleanup(cx, move || {
        log!("Cleaning up closure");
        // This is needed to let the callback live long enough.
        // https://rustwasm.github.io/wasm-bindgen/examples/closures.html
        // https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html
        let _cleanup = resize_callback;
    });

    on_mount
}

fn try_extract_resize_observer_size(box_size_array: Array) -> Option<ResizeEvent> {
    if box_size_array.length() > 0 {
        if let Ok(resize_observer_size) =
            TryInto::<ResizeObserverSize>::try_into(box_size_array.get(0))
        {
            return Some(ResizeEvent {
                block_size: resize_observer_size.block_size(),
                inline_size: resize_observer_size.inline_size(),
            });
        }
    }
    None
}
