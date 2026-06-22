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
    pub fn get_print_stdout<T, E: backend::WasmEngine>(
        instance: &Instance,
        _store: &mut Store<T, E>,
    ) -> Result<TypedFunc<String, ()>> {
        let interface = instance
            .exports()
            .instance(&INTERFACE_NAME.try_into().unwrap())
            .ok_or_else(|| anyhow!("Interface not found"))?;

        interface
            .func("print-stdout")
            .ok_or_else(|| anyhow!("Function 'print-stdout' not found"))?
            .typed::<String, ()>()
    }

    #[allow(clippy::type_complexity)]
    pub fn get_print_stderr<T, E: backend::WasmEngine>(
        instance: &Instance,
        _store: &mut Store<T, E>,
    ) -> Result<TypedFunc<String, ()>> {
        let interface = instance
            .exports()
            .instance(&INTERFACE_NAME.try_into().unwrap())
            .ok_or_else(|| anyhow!("Interface not found"))?;

        interface
            .func("print-stderr")
            .ok_or_else(|| anyhow!("Function 'print-stderr' not found"))?
            .typed::<String, ()>()
    }

    #[allow(clippy::type_complexity)]
    pub fn get_read_stdin<T, E: backend::WasmEngine>(
        instance: &Instance,
        _store: &mut Store<T, E>,
    ) -> Result<TypedFunc<(), String>> {
        let interface = instance
            .exports()
            .instance(&INTERFACE_NAME.try_into().unwrap())
            .ok_or_else(|| anyhow!("Interface not found"))?;

        interface
            .func("read-stdin")
            .ok_or_else(|| anyhow!("Function 'read-stdin' not found"))?
            .typed::<(), String>()
    }

}

