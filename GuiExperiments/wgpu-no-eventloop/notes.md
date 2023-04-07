
# Notes on how to use wgpu + wasm without winit, i.e., without an event loop

There is either a "generic" solution, which doesn't involve writing any wasm/web_sys specific
code, or an "explicit" solution that operates directly on the canvas element.

**Generic solution**

This is based on the [raw-window-handle](https://docs.rs/raw-window-handle/latest/raw_window_handle/) crate
and its `WebWindowHandle` or `WebDisplayHandle` traits.

These traits require to assign a unique ID to the canvas elements, and custom data attribute must be attached
to the DOM element. This allows wgpu to identify and load the canvas under that ID.
This happens in [`backend/web.rs`](https://github.com/gfx-rs/wgpu/blob/7fd129a5356a669914340d01707099a6fa31f098/wgpu/src/backend/web.rs#L839-L862)
in `instance_create_surface`: `doc.query_selector_all` re-identifies the canvas via its ID, and then calls
`instance_create_surface_from_canvas`.

When using the webgl target instead of the actual webgpu implementation something similar happens in
[`wgpu-hal/src/gles/web.rs`](https://github.com/gfx-rs/wgpu/blob/trunk/wgpu-hal/src/gles/web.rs)
(note that `create_surface_from_canvas` in this case instantiates a `webgl2` context).

I found an existing usage of this pattern [here](https://github.com/open-mv-sandbox/ptero/blob/f13bbf6f1d9a70068c132137af9a5ae112ed9276/crates/dacti-viewer-js/src/surface.rs)
on GitHub.


**Direct solution**

In a wasm specific use case it feels a bit awkward to implement these ID based traits, i.e., extracting a surface
directly from the canvas is just simpler. This has been asked [here](https://github.com/gfx-rs/wgpu/discussions/2893)
as well. Apparently it is possible to call `Instance::create_surface_from_canvas` directly, when compiling for the
wasm target. I was a bit confused if this function really exists because [the docs](https://docs.rs/wgpu/latest/wgpu/struct.Instance.html)
do not seem to show such conditionally compiled functions.

Also, VSCode makes it a bit awkward to work with wasm-only code branches, because by default it insists that they
don't exist. To work around that it helps to create a workspace specific `settings.json` containing:

```
{
  "rust-analyzer.cargo.target": "wasm32-unknown-unknown",
}
```

