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
    pub fn get_get_env_var_all<T, E: backend::WasmEngine>(
        instance: &Instance,
        _store: &mut Store<T, E>,
    ) -> Result<TypedFunc<(), Vec<(String, String)>>> {
        let interface = instance
            .exports()
            .instance(&INTERFACE_NAME.try_into().unwrap())
            .ok_or_else(|| anyhow!("Interface not found"))?;

        interface
            .func("get-env-var-all")
            .ok_or_else(|| anyhow!("Function 'get-env-var-all' not found"))?
            .typed::<(), Vec<(String, String)>>()
    }

    #[allow(clippy::type_complexity)]
    pub fn get_get_env_var<T, E: backend::WasmEngine>(
        instance: &Instance,
        _store: &mut Store<T, E>,
    ) -> Result<TypedFunc<String, Option<String>>> {
        let interface = instance
            .exports()
            .instance(&INTERFACE_NAME.try_into().unwrap())
            .ok_or_else(|| anyhow!("Interface not found"))?;

        interface
            .func("get-env-var")
            .ok_or_else(|| anyhow!("Function 'get-env-var' not found"))?
            .typed::<String, Option<String>>()
    }

    #[allow(clippy::type_complexity)]
    pub fn get_get_program_name<T, E: backend::WasmEngine>(
        instance: &Instance,
        _store: &mut Store<T, E>,
    ) -> Result<TypedFunc<(), String>> {
        let interface = instance
            .exports()
            .instance(&INTERFACE_NAME.try_into().unwrap())
            .ok_or_else(|| anyhow!("Interface not found"))?;

        interface
            .func("get-program-name")
            .ok_or_else(|| anyhow!("Function 'get-program-name' not found"))?
            .typed::<(), String>()
    }

    #[allow(clippy::type_complexity)]
    pub fn get_get_cli_args<T, E: backend::WasmEngine>(
        instance: &Instance,
        _store: &mut Store<T, E>,
    ) -> Result<TypedFunc<(), Vec<String>>> {
        let interface = instance
            .exports()
            .instance(&INTERFACE_NAME.try_into().unwrap())
            .ok_or_else(|| anyhow!("Interface not found"))?;

        interface
            .func("get-cli-args")
            .ok_or_else(|| anyhow!("Function 'get-cli-args' not found"))?
            .typed::<(), Vec<String>>()
    }

}

