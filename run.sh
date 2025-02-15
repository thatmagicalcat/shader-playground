#!/bin/sh

RUSTFLAGS=""
wasm-pack build --out-dir pkg --target web
