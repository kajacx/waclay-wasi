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

if ! echo -n $'line1\nline2' | cargo --quiet run > output-stdout.txt 2> output-stderr.txt; then
    cat output-stdout.txt
    cat output-stderr.txt >&2

    exit 1
fi

diff -u output-stdout.txt.snap output-stdout.txt
diff -u output-stderr.txt.snap output-stderr.txt

cd ..
