# RLOG

Implementation of logistics models/algorithms using Rust, WebAssembly and VueJS.

Starting with exponential smoothing.

## Build

Run `wasm-pack build --dev` in the project root to perform a development build.

Run `wasm-pack build --release` to perform a release build (default).

The compiled WebAssembly code will reside in `./pkg` wrapped as a npm package ready to be published / imported in your web application / etc.
