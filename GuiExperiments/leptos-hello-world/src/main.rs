mod no_macros;
mod style;
mod svelte_like_styles;
mod svelte_like_styles_via_stylers;
mod timer;

use crate::no_macros::{NoMacrosSimpleCounter, NoMacrosSimpleCounterProps};
use crate::svelte_like_styles::{Parent, ParentProps};
use crate::svelte_like_styles_via_stylers::{ParentV2, ParentV2Props};
use crate::timer::{Timer, TimerProps};
use lazy_static::lazy_static;
use leptos::*;
use style::{create_style, css};

#[component]
pub fn SimpleCounter(cx: Scope, initial_value: i32) -> impl IntoView {
    let (value, set_value) = create_signal(cx, initial_value);

    let clear = move |_| set_value(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    view! {
        cx,
        <div class=&*STYLE>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            // <span>"Value: " {value} "!"</span>
            <ValueDisplay value />
            <button on:click=increment>"+1"</button>
        </div>
    }
}

#[component]
pub fn ValueDisplay(cx: Scope, value: ReadSignal<i32>) -> impl IntoView {
    view! {
        cx,
        <span>"Value: " {value} "!"</span>
    }
}

// const STYLE: String = create_style();

// lazy_static! {
//     static ref STYLE: String = create_style("color: #00F;");
// }

// fn style() -> &'static str {
//     &*STYLE
// }

css!(
    STYLE,
    r#"
    color: #0F0;
    font-size: 20px;

    & button {
        padding: 10px;
        color: #F00;
        font-size: 30px;
    }
    "#
);

css!(
    STYLE2,
    r#"
    color: #0F0;

    & button {
        padding: 10px;
        color: #F00;
    }
    "#
);

/*
Note on that ugly `MaybeSignal::derive`:

Would it be possible to implement From directly for functions/closures?

This seems tricky. The problem is that the From implementation needs to be
generic in the function type. Something like:

```
impl<F, T> From<F> for MaybeSignal<T>
where
    F: Fn() -> T
{
    fn from(value: &str) -> Self {
        unimplemented!()
    }
}
```

The problem is that this is not allowed, because there is also an implementation
`impl<T> From<T> ...` which matches as well, and the compiler cannot know which one
should be used. See:

https://stackoverflow.com/questions/37347311/how-is-there-a-conflicting-implementation-of-from-when-using-a-generic-type

Another try was to use a function wrapper with boxing like this:

```
struct FuncWrapper<T>(Box<dyn Fn() -> T>);

impl<T> From<FuncWrapper<T>> for MaybeSignal<T> {
    fn from(value: FuncWrapper<T>) -> Self {
        Self::Dynamic(Signal::derive(cx, value.0))
    }
}
```
But that also cannot work, because deriving a signal need knowledge of the reactive context,
which the closure doesn't know.

In the end it may simple be best to get used to using a helper function to derive signals.

```
let double_value = create_derived_signal(cx, move |_| value() * 2);
```

which would be in line with other `create_...` functions.

If that passing around of `cx` gets annoying, one could also add macro versions `create_signal!(...)`,
`create_memo!(...)`, `create_derived_signal!(...)` that implicitly assume that the context is called
`cx`. A bit nasty for saving 3 characters though (avoids the `cx, ` but adds an `!`).

*/
#[component]
pub fn PropsPassingExperiment(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);

    let plain_t = 42;
    let double_value = move || value() * 2;
    let double_value_memoed = create_memo(cx, move |_| value() * 2);

    let increment = move |_| set_value.update(|value| *value += 1);

    view! {
        cx,
        <div>
        <button on:click=increment>"increment"</button>

        <SubComponent1 value={value} />
        <SubComponent1 value={42} />
        <SubComponent1 value={plain_t} />
        // <SubComponent value={double_value} /> // This doesn't work and requires explicit deriving, see comment above.
        <SubComponent1 value={MaybeSignal::derive(cx, double_value)} />
        <SubComponent1 value={double_value_memoed} />

        // <SubComponent2 value={String::from("Hello World")} /> // Using plain Signal prevents constants use as expected.
        // As a work-around users would have to create dummy signals, a bit tedious.
        <SubComponent2 value={create_signal(cx, "Hello World".into()).0} />
        <SubComponent2 value={Signal::derive(cx, move || format!("stringified: {}", value()))} />
        </div>
    }
}

#[component]
pub fn SubComponent1(
    cx: Scope,
    /// This prop is either constant or reactive.
    #[prop(into)]
    value: MaybeSignal<i32>,
) -> impl IntoView {
    log!("Rendering SubComponent1 with {:?}", value);
    view! {
        cx,
        <div>{value}</div>
    }
}

#[component]
pub fn SubComponent2(
    cx: Scope,
    /// This prop is must be reactive -- unnecessarily restrictive?
    #[prop(into)]
    value: Signal<String>,
) -> impl IntoView {
    log!("Rendering SubComponent2 with {:?}", value);
    view! {
        cx,
        <div>{value}</div>
    }
}

#[component]
pub fn MountTest(cx: Scope) -> impl IntoView {
    let (mounted, set_mounted) = create_signal(cx, false);
    let toggle = move |_| set_mounted.update(|value| *value = !*value);

    view! {
        cx,
        <div>
        <button on:click=toggle>"toggle"</button>
        <Show when={mounted} fallback=|_| ()>
          <SubComponent1 value={42}/>
        </Show>
        </div>
    }
}

pub fn main() {
    log!("Mounting to body...");
    // add_css().expect("Failed to add CSS");
    mount_to_body(|cx| {
        view! {
            cx,
            <SimpleCounter initial_value=3 />
            <NoMacrosSimpleCounter initial_value=5 />
            <PropsPassingExperiment/>
            <MountTest/>
            <Parent />
            <ParentV2 />
            <Timer />
        }
    })
}
