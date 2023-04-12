use leptos::html::*;
use leptos::*;

#[component]
pub fn NoMacrosSimpleCounter(cx: Scope, initial_value: i32) -> impl IntoView {
    let (value, set_value) = create_signal(cx, initial_value);

    let clear = move |_| set_value(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    div(cx)
        .child(button(cx).on(ev::click, clear).child("Clear"))
        .child(button(cx).on(ev::click, decrement).child("-1"))
        .child(ValueDisplay(cx, ValueDisplayProps { value }))
        .child(button(cx).on(ev::click, increment).child("+1"))
}

#[component]
pub fn ValueDisplay(cx: Scope, value: ReadSignal<i32>) -> impl IntoView {
    span(cx).child("Value: ").child(value).child("!")
}
