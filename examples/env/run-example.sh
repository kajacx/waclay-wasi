#!/usr/bin/sh
set -e

# Run from this directory

cd guest
cargo build --target wasm32-wasip2
cd ..

echo "// This is a PARTIAL wasip2 wit file generated from the env example!" > wasip2-env-partial.wit
echo >> wasip2-env-partial.wit
wasm-tools component wit guest/target/wasm32-wasip2/debug/example_env_guest.wasm >> wasip2-env-partial.wit

wit-bindgen-wcl ./env.wit ./host/src/bindings.rs

cd host
ENV1=var1 ENV2=var2 cargo run arg1 arg2
cd ..
