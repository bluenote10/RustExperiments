Following:
- https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html
- https://github.com/rustwasm/wasm-bindgen/tree/main/examples/without-a-bundler

Serving directly from the file system doesn't work, because neighboring files
have to be loaded. So development work-flow would rely on:

```sh
python -m http.server
```