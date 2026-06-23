#!/usr/bin/sh
set -e

# Run from this directory

cd guest
cargo build --target wasm32-wasip2
cd ..

echo "// This is a PARTIAL wasip2 wit file generated from the stdio example!" > wasip2-stdio-partial.wit
echo >> wasip2-stdio-partial.wit
wasm-tools component wit guest/target/wasm32-wasip2/debug/example_stdio_guest.wasm >> wasip2-stdio-partial.wit

wit-bindgen-wcl ./stdio.wit ./host/src/bindings.rs

cd host
echo 'hello
world' | cargo run
cd ..
