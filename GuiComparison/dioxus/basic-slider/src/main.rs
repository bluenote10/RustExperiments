#![allow(non_snake_case)]

mod basic_slider;

use dioxus::prelude::*;
use log::LevelFilter;

use basic_slider::BasicSlider;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { BasicSlider {} }
}
