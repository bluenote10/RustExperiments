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
- Elm architecture based
- VDOM based


### Iced

- Elm architecture based


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


### Druid

https://github.com/linebender/druid

- Deprecated, successor is Xilem


### Xilem

https://github.com/linebender/xilem

- Looks still prototypical, but sounds like the design may be sophisticated.
- To read, blog post: https://raphlinus.github.io/rust/gui/2022/05/07/ui-architecture.html


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


# WASM notes

There seems to be a general choice: bundler vs no bundler.

So far experiments with Webpack felt bad.

In general it should be possible to use Vite as well. This plugin looks relatively
well maintained and mentions hot reloading (although not very much documented):
- https://github.com/rwasm/vite-plugin-rsw

There is also https://github.com/nshen/vite-plugin-wasm-pack but it doesn't look very
actively maintained.


**Is it possible to use C libraries in WASM**

https://stackoverflow.com/questions/51666736/how-do-i-use-a-c-library-in-a-rust-library-compiled-to-webassembly

That seems to be one of the key problem that people are trying to solve currently.

In theory it can be possible to re-compile the C library with emscripten to bring it to WASM itself,
then it should be possible to use it. However in practice that's not always straightforward apparently.


# Web app frameworks / HTTP servers

Overviews:
- https://www.arewewebyet.org/topics/frameworks/
- https://kerkour.com/rust-web-framework-2022
- https://github.com/flosse/rust-web-framework-comparison

Some entries found actually in the [LiveView examples](https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples)
of Dioxus.

Some benchmarks:
- https://web-frameworks-benchmark.netlify.app/result?l=rust
- https://github.com/programatik29/rust-web-benchmarks/blob/master/result/hello-world.md


In terms of GitHub stars:
Rocket (20.1k) > actix-web (16.9k) > Axum (9.2k) > warp (7.8k) > salvo (1.4k)


## actix-web

https://github.com/actix/actix-web


## Axum

https://github.com/tokio-rs/axum

- Part of Tokio project

## Salvo

https://github.com/salvo-rs/salvo


## Rocket

https://github.com/SergioBenitez/Rocket


## warp

https://github.com/seanmonstar/warp


## hyper

https://github.com/hyperium/hyper

> hyper is a relatively low-level library, meant to be a building block for libraries and applications.

> If you are looking for a convenient HTTP client, then you may wish to consider reqwest.
> If you are looking for a convenient HTTP server, then you may wish to consider warp.
> Both are built on top of this library.

