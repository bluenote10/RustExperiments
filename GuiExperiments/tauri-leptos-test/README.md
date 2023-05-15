Inspired by:
https://github.com/michalvavra/tauri-leptos-example


To start dev server:

```sh
npx @tauri-apps/cli dev
```

Note that `npx tauri` picks up the [wrong](https://www.npmjs.com/package/tauri) (outdated alpha) tauri package.

Regarding IPC / serialization overhead:
- https://github.com/tauri-apps/tauri/discussions/1336#discussioncomment-456047
- https://github.com/tauri-apps/tauri/discussions/5690
- https://github.com/tauri-apps/wry/issues/767

Status of integration `tauri-bindgen`: `tauri-bindgen` wasn't even released on crates.io, so waiting for a release...
