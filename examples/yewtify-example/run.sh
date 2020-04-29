cp static/* pkg
YEWTIFY_OUT=$(pwd)/pkg RUST_LOG=warn wasm-pack build --target web
rollup ./pkg/main.js --format iife --file ./pkg/bundle.js
miniserve pkg
