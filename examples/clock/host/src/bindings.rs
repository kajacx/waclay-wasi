// AUTO-GENERATED WIT BINDINGS for wasm-component-layer
// DO NOT EDIT - Regenerate using wit-bindgen-wcl

#![allow(dead_code, unused_imports, ambiguous_glob_reexports)]

use anyhow::*;
use waclay::*;
use wasm_runtime_layer::{backend};


// ========== Type Definitions ==========

// ========== Guest Exports ==========

pub mod exports_funcs {
    use super::*;

    pub const INTERFACE_NAME: &str = "waclay-wasi:examples/funcs@0.1.0";

    #[allow(clippy::type_complexity)]
    pub fn get_get_wall_clock<T, E: backend::WasmEngine>(
        instance: &Instance,
        _store: &mut Store<T, E>,
    ) -> Result<TypedFunc<(), u64>> {
        let interface = instance
            .exports()
            .instance(&INTERFACE_NAME.try_into().unwrap())
            .ok_or_else(|| anyhow!("Interface not found"))?;

        interface
            .func("get-wall-clock")
            .ok_or_else(|| anyhow!("Function 'get-wall-clock' not found"))?
            .typed::<(), u64>()
    }

    #[allow(clippy::type_complexity)]
    pub fn get_get_time_elapsed<T, E: backend::WasmEngine>(
        instance: &Instance,
        _store: &mut Store<T, E>,
    ) -> Result<TypedFunc<(), u64>> {
        let interface = instance
            .exports()
            .instance(&INTERFACE_NAME.try_into().unwrap())
            .ok_or_else(|| anyhow!("Interface not found"))?;

        interface
            .func("get-time-elapsed")
            .ok_or_else(|| anyhow!("Function 'get-time-elapsed' not found"))?
            .typed::<(), u64>()
    }

}

