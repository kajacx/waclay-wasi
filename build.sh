#!/usr/bin/sh
set -e

# Run from this directory

# This will generate the bindings but they are kind of broken
# wit-bindgen-wcl ./wasip2-partial.wit ./src/bindings.rs
cargo run --manifest-path ../waclay/Cargo.toml -p wit-bindgen-wcl ./wasip2-partial.wit ./src/bindings.rs
rustfmt ./src/bindings.rs

# Manual fixes round 1: Imports
# sed -i 's/^use anyhow::\*;$/use waclay::anyhow::*;/' ./src/bindings.rs;
# sed -i 's/^use wasm_runtime_layer::\*;$/use waclay::wasm_runtime_layer::*;/' ./src/bindings.rs;
# sed -i '9i use crate::ResourceConvert;' ./src/bindings.rs;

# Manual fixes round 2: Custom resources
sed -i 's/\bError\b/crate::WasiP2ErrorResource/g' ./src/bindings.rs
sed -i 's/\bInputStream\b/crate::WasiP2InputStreamResource/g' ./src/bindings.rs
sed -i 's/\bOutputStream\b/crate::WasiP2OutputStreamResource/g' ./src/bindings.rs
sed -i 's/\bPollable\b/crate::WasiP2PollableResource/g' ./src/bindings.rs
sed -i 's/\bTerminalInput\b/crate::WasiP2TerminalInputResource/g' ./src/bindings.rs
sed -i 's/\bTerminalOutput\b/crate::WasiP2TerminalOutputResource/g' ./src/bindings.rs

# Cleanup after round 2
sed -i 's/^pub struct crate::WasiP2/pub struct /' ./src/bindings.rs
sed -i 's/^impl crate::WasiP2/impl /' ./src/bindings.rs

# Manual fixes round 3: Bad variable names
# sed -i 's/\b_host_interface\b/host_interface/g' ./src/bindings.rs

# Manual fixes round 4: Resources again, this time conversion
# sed -i 's/result.into_value()?;/result.to_value(ctx)?;/g' ./src/bindings.rs
# sed -i 's/Resource::from_value(&params\[0])?;/Resource::from_value(\&mut ctx, params[0].clone())?;/g' ./src/bindings.rs

cargo build
