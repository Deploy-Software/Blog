#!/bin/bash

cd app
wasm-pack build --target web --out-name wasm --out-dir ../server/static
cd ../server
cargo run
