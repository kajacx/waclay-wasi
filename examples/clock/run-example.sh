#!/usr/bin/sh
set -e

# Run from this directory

cd guest
cargo build --target wasm32-wasip2
cd ..

echo "// This is a PARTIAL wasip2 wit file generated from the clock example!" > wasip2-clock-partial.wit
echo >> wasip2-clock-partial.wit
wasm-tools component wit guest/target/wasm32-wasip2/debug/example_clock_guest.wasm >> wasip2-clock-partial.wit

wit-bindgen-wcl ./clock.wit ./host/src/bindings.rs

cd host
cargo run
cd ..
