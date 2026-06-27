use waclay::*;
use waclay_wasi::{AsWasiP2Ctx, WasiP2Ctx};

mod bindings;

// The bytes of the component.
const WASM: &[u8] = include_bytes!("../../guest/target/wasm32-wasip2/debug/example_env_guest.wasm");

struct StoreData {
    ctx: WasiP2Ctx,
}

impl AsWasiP2Ctx for StoreData {
    fn as_wasi_ref(&self) -> &WasiP2Ctx {
        &self.ctx
    }

    fn as_wasi_mut(&mut self) -> &mut WasiP2Ctx {
        &mut self.ctx
    }
}

pub fn main() {
    // Prepare store, component and exports

    let engine = Engine::new(wasmi_runtime_layer::Engine::default());
    let mut store = Store::new(
        &engine,
        StoreData {
            ctx: WasiP2Ctx::new(),
        },
    );

    let mut linker = Linker::default();
    waclay_wasi::add_to_linker(&mut linker, &mut store).unwrap();

    let component = Component::new(&engine, WASM).unwrap();

    // PROBLEM!!!
    // Wasi will fetch the env vars ONCE on startup / when they are requested,
    // and will not re-fetch them on subsequent queries.
    // So we have to re-do the instance and re-fetch the exports for every use case.

    // By default, there are no env variables, cli arguments, and program name is an empty string

    let instance = linker.instantiate(&mut store, &component).unwrap();

    let get_env_var_all =
        bindings::exports_funcs::get_get_env_var_all(&instance, &mut store).unwrap();
    let get_program_name =
        bindings::exports_funcs::get_get_program_name(&instance, &mut store).unwrap();
    let get_cli_args = bindings::exports_funcs::get_get_cli_args(&instance, &mut store).unwrap();

    let env_all = get_env_var_all.call(&mut store, ()).unwrap();
    assert_eq!(env_all, vec![]);

    let name = get_program_name.call(&mut store, ()).unwrap();
    assert_eq!(name, "");

    let args = get_cli_args.call(&mut store, ()).unwrap();
    assert_eq!(args, Vec::<String>::new());

    // Inherit environment variables and arguments from host

    store
        .data_mut()
        .as_wasi_mut()
        .clear_all()
        .inherit_environment_vars(["ENV1"], waclay_wasi::InsertionMode::Merge)
        .inherit_program_name()
        .inherit_cli_arguments();

    let instance = linker.instantiate(&mut store, &component).unwrap();

    let get_env_var = bindings::exports_funcs::get_get_env_var(&instance, &mut store).unwrap();
    let get_program_name =
        bindings::exports_funcs::get_get_program_name(&instance, &mut store).unwrap();
    let get_cli_args = bindings::exports_funcs::get_get_cli_args(&instance, &mut store).unwrap();

    let env1 = get_env_var.call(&mut store, "ENV1".to_string()).unwrap();
    assert_eq!(env1, Some("value1".to_string()));

    let env2 = get_env_var.call(&mut store, "ENV2".to_string()).unwrap();
    assert_eq!(env2, None);

    let program_name = get_program_name.call(&mut store, ()).unwrap();
    assert!(
        [
            "target\\debug\\example-stdio-host.exe",
            "target/debug/example-stdio-host"
        ]
        .contains(&program_name.as_str()),
        "Unexpected program name: {program_name}"
    );

    let args = get_cli_args.call(&mut store, ()).unwrap();
    assert_eq!(args, vec!["arg1".to_string(), "arg2".to_string()]);

    // Set custom environment vars and arguments

    store
        .data_mut()
        .as_wasi_mut()
        .clear_all()
        .set_environment_vars(
            [("CUSTOM1", "custom_value1")],
            waclay_wasi::InsertionMode::Merge,
        )
        .set_program_name("waclay_guest")
        .set_cli_arguments(["custom_arg1", "custom_arg2"]);

    let instance = linker.instantiate(&mut store, &component).unwrap();

    let get_env_var = bindings::exports_funcs::get_get_env_var(&instance, &mut store).unwrap();
    let get_program_name =
        bindings::exports_funcs::get_get_program_name(&instance, &mut store).unwrap();
    let get_cli_args = bindings::exports_funcs::get_get_cli_args(&instance, &mut store).unwrap();

    let env1 = get_env_var.call(&mut store, "CUSTOM1".to_string()).unwrap();
    assert_eq!(env1, Some("custom_value1".to_string()));

    let env2 = get_env_var.call(&mut store, "ENV2".to_string()).unwrap();
    assert_eq!(env2, None);

    let program_name = get_program_name.call(&mut store, ()).unwrap();
    assert_eq!(program_name, "waclay_guest");

    let args = get_cli_args.call(&mut store, ()).unwrap();
    assert_eq!(
        args,
        vec!["custom_arg1".to_string(), "custom_arg2".to_string()]
    );
}
