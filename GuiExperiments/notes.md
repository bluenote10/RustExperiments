# GUI frameworks

Benchmarks: https://krausest.github.io/js-framework-benchmark/current.html

Overview:
- Yew or Dioxus can look a lot like React.
- Sycamore or Leptos apps will both look a lot like SolidJS apps.

Misc general resources / overviews / discussions:
- https://www.reddit.com/r/rust/comments/yd9ngs/worried_about_modern_rust_gui_libraries/
  Worries about VDOM based approaches with discussions on alternatives.


## Yew

- Most mature / oldest framework
- ELM architecture based
- VDOM based



## Dioxus

https://dioxuslabs.com/

- VDOM based
- Surprisingly good performance for a VDOM approach
- Many target systems, not just web.

This blob post summarizes some design aspects and performance aspects well:
https://dioxuslabs.com/blog/templates-diffing/


## Sycamore

- More mature than Leptos
- No VDOM, SolidJS architecture


## Leptos

https://github.com/leptos-rs/leptos

- Comes with server side rendering in mind.
- No VDOM, SolidJS architecture


## Slint

https://github.com/slint-ui/slint

- Not DOM based, but renders on canvas more like egui.
- Fonts are ugly, no subpixel hinting.
- Many backends, not just web.
- Uses a special markup language for abstraction.
- UI components not very nice.


## Sledgehammer

https://github.com/demonthos/sledgehammer

- Looks very much optimized for the krausest benchmark.
- States that wasm-bindgen/web-sys is more general/ergonomic (are pure manual DOM modifications ergonomic?)

