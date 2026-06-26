#!/usr/bin/sh
set -e

# Run from this directory

cd guest
cargo build --target wasm32-wasip2
cd ..

echo "// This is a PARTIAL wasip2 wit file generated from the random example!" > wasip2-random-partial.wit
echo >> wasip2-random-partial.wit
wasm-tools component wit guest/target/wasm32-wasip2/debug/example_random_guest.wasm >> wasip2-random-partial.wit

wit-bindgen-wcl ./random.wit ./host/src/bindings.rs

cd host
cargo run
cd ..
