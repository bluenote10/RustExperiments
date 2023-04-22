use leptos::*;
use wasm_bindgen::{prelude::Closure, JsCast};

#[component]
pub fn Timer(cx: Scope) -> impl IntoView {
    // count_a updates with a fixed interval of 1000 ms, whereas count_b has a dynamic
    // update interval.
    let (count_a, set_count_a) = create_signal(cx, 0_i32);
    let (count_b, set_count_b) = create_signal(cx, 0_i32);

    let (interval, set_interval) = create_signal(cx, 1000_i32);

    use_interval(cx, 1000, move || {
        set_count_a.update(|c| *c = *c + 1);
    });
    use_interval(cx, interval, move || {
        set_count_b.update(|c| *c = *c + 1);
    });

    view! {
        cx,
        <div>
            <div>"Count A (fixed interval of 1000 ms)"</div>
            <div>{count_a}</div>
            <div>"Count B (dynamic interval, currently " {interval} "ms )"</div>
            <div>{count_b}</div>
            <input prop:value=interval on:input=move |ev| {
                if let Ok(value) = event_target_value(&ev).parse::<i32>() {
                    set_interval(value);
                }
            }/>
        </div>
    }
}

//pub fn use_interval<T>(cx: Scope, interval_millis: T, f: impl Fn() -> () + 'static)
pub fn use_interval<T, F>(cx: Scope, interval_millis: T, f: F)
where
    F: Fn() -> () + 'static,
    T: Into<MaybeSignal<i32>> + 'static,
{
    let js_callback: Closure<dyn Fn()> = Closure::new(move || {
        log!("Running timer");
        f();
    });
    let js_callback_clone = js_callback.as_ref().clone();

    let interval_millis = interval_millis.into();

    create_effect(cx, move |_| {
        let window = web_sys::window().unwrap();
        let interval_id = window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                js_callback_clone.unchecked_ref(),
                interval_millis(),
            )
            .expect("Failed set interval");

        on_cleanup(cx, move || {
            let window = web_sys::window().unwrap();
            window.clear_interval_with_handle(interval_id);
        })
    });

    on_cleanup(cx, move || {
        let _keep_alive = js_callback;
    });
}

/*

// Note that something like the following cannot really work, because the outer closure
// (the one from the create_effect) can run multiple times, and therefore it cannot really
// move `f` multiple times into the closure passed to Closure::new.

pub fn use_interval<T>(cx: Scope, interval_millis: T, f: impl Fn() -> () + 'static)
where
    //F: Fn() -> () + 'static,
    T: Into<MaybeSignal<i32>>,
{
    create_effect(cx, move |_| {
        /*
        let js_callback = Closure::<dyn Fn()>::new(move || {
            log!("Running timer");
            f();
        });
         */

        // let js_callback = Closure::<dyn FnOnce()>::new(move || {
        //     log!("Running timer");
        //     f();
        // });

        let cb: Closure<dyn Fn() + 'static> = Closure::new(move || {
            web_sys::console::log_1(&"interval elapsed!".into());
            f();
        });

        let window = web_sys::window().unwrap();
        let interval_id = window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                // Note this method call, which uses `as_ref()` to get a `JsValue`
                // from our `Closure` which is then converted to a `&Function`
                // using the `JsCast::unchecked_ref` function.
                cb.as_ref().unchecked_ref(),
                1_000,
            )
            .expect("Failed set interval");
    });
}
*/
