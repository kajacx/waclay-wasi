#!/usr/bin/sh
set -e

# Run from this directory

cd guest
cargo build --target wasm32-wasip2
cd ..

echo "// This is a PARTIAL wasip2 wit file generated from the filesystem example!" > wasip2-filesystem-partial.wit
echo >> wasip2-filesystem-partial.wit
wasm-tools component wit guest/target/wasm32-wasip2/debug/example_filesystem_guest.wasm >> wasip2-filesystem-partial.wit

wit-bindgen-wcl ./filesystem.wit ./host/src/bindings.rs

cd host
cargo run
cd ..
