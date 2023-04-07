./client_server/wasm-client/watch_build.sh
#!/bin/bash

cd $(dirname "$0")

cargo watch --clear --why --delay 0.2 -s "wasm-pack build --target web" -s "notify-send 'Compilation done'"
