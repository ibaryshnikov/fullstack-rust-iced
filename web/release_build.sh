#!/bin/bash

set -e

cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web --out-dir dist/pkg target/wasm32-unknown-unknown/release/web.wasm
