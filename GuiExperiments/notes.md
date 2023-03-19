# GUI frameworks

Benchmarks: https://krausest.github.io/js-framework-benchmark/current.html

Overview:
- Yew or Dioxus can look a lot like React.
- Sycamore or Leptos apps will both look a lot like SolidJS apps.

Misc general resources / overviews / discussions:
- https://www.reddit.com/r/rust/comments/yd9ngs/worried_about_modern_rust_gui_libraries/
  Worries about VDOM based approaches with discussions on alternatives.

## DOM-based

### Yew

- Most mature / oldest framework
- ELM architecture based
- VDOM based



### Dioxus

https://dioxuslabs.com/

- VDOM based
- Surprisingly good performance for a VDOM approach
- Many target systems, not just web.

This blob post summarizes some design aspects and performance aspects well:
https://dioxuslabs.com/blog/templates-diffing/


### Sycamore

- More mature than Leptos
- No VDOM, SolidJS architecture


### Leptos

https://github.com/leptos-rs/leptos

- Comes with server side rendering in mind.
- No VDOM, SolidJS architecture


### Sledgehammer

https://github.com/demonthos/sledgehammer

- Looks very much optimized for the krausest benchmark.
- States that wasm-bindgen/web-sys is more general/ergonomic (are pure manual DOM modifications ergonomic?)


## Non-DOM based

## egui

- The Rust equivalent of dear-imgui, but actually looking almost more advanced.

## Bevy

- Rather a game engine.
- UI functionality looks pretty basic.
- Quite interesting: It looks like the Dioxus and Bevy authors have joined forces on a high performance
  layout UI layout library (I assume Dioxus mainly needs it in the desktop mode to match the DOM semantics).
  Originally on reddit this was called "stretch2" but I think it now is named "taffy":
  https://github.com/DioxusLabs/taffy
  https://www.reddit.com/r/rust/comments/umwjt4/bevy_and_dioxus_are_collaborating_on_stretch2_a/


## Slint

https://github.com/slint-ui/slint

- Not DOM based, but renders on canvas more like egui.
- Fonts are ugly, no subpixel hinting.
- Many backends, not just web.
- Uses a special markup language for abstraction.
- UI components not very nice.


# WebGPU

To test if browser supports it:
- https://webgpu.github.io/webgpu-samples/samples/helloTriangle
- https://webkit.org/demos/webgpu/hello-triangle.html
- https://webkit.org/demos/webgpu/hello-cube.html
- https://hello-webgpu-compute.glitch.me/
- https://toji.github.io/webgpu-clustered-shading/


This gives an overview of support status:
- https://github.com/gpuweb/gpuweb/wiki/Implementation-Status

Note that stable Firefox has the settings mentioned in
https://stackoverflow.com/questions/73706354/how-to-try-webgpu-in-firefox-nightly-now-in-fall-of-2022
in `about:config` but it looks like you just get a different kind of
error when enabling it with all the demos/tests.

General showcases (mostly Desktop apps for now?): https://wgpu.rs/