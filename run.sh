#!/bin/sh

RUSTFLAGS=""
wasm-pack build --dev --out-dir pkg --target web
python3 -m http.server -d .
