#!/bin/sh

RUSTFLAGS=""
wasm-pack build --out-dir pkg --target web

rm pkg/README.md
rm pkg/.gitignore
