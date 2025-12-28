#!/bin/bash

set -e

cd dist/pkg
wasm-opt web_bg.wasm -o web_bg.wasm -O
