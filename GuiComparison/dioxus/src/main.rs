#![allow(non_snake_case)]
#![allow(unused_imports)]

mod basic_reactivity;
mod basic_slider;
mod locked_sliders;

use dioxus::prelude::*;
use log::LevelFilter;

use basic_reactivity::ReactiveParent;
use basic_slider::BasicSlider;
use locked_sliders::LockedSliders;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { BasicSlider {} }
}
