Mainly following:
- https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html
- https://github.com/rustwasm/wasm-bindgen/tree/main/examples/hello_world

It works in the dev server mode, but I don't even understand where the index.html
is coming from.

Trying to split the structure into a `rust` + `web` folder failed, probably because
the import goes outside of the `node_modules` structure, and node.js always tries to
resolve things in a parent `node_modules` folder. At least the error indicated that
the issue is that from the perspective of `rust/pkg` there is no `node_modules` anywhere
in the parent folder structure.
