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
            ctx: waclay_wasi::WasiP2Ctx::new(),
        },
    );

    let mut linker = Linker::default();
    waclay_wasi::add_to_linker(&mut linker, &mut store).unwrap();

    let component = Component::new(&engine, WASM).unwrap();
    let instance = linker.instantiate(&mut store, &component).unwrap();

    let get_get_env_var_all =
        bindings::exports_funcs::get_get_env_var_all(&instance, &mut store).unwrap();
    // let get_get_env_var = bindings::exports_funcs::get_get_env_var(&instance, &mut store).unwrap();
    let get_get_program_name =
        bindings::exports_funcs::get_get_program_name(&instance, &mut store).unwrap();
    let get_get_cli_args =
        bindings::exports_funcs::get_get_cli_args(&instance, &mut store).unwrap();

    // Ignoring std io and pretending it's closed is already the default behaviour,
    // but you can call "clear" methods to make sure in case wasi ctx was modified before.

    let env_all = get_get_env_var_all.call(&mut store, ()).unwrap();
    assert_eq!(env_all, vec![]);

    let name = get_get_program_name.call(&mut store, ()).unwrap();
    assert_eq!(name, "");

    let args = get_get_cli_args.call(&mut store, ()).unwrap();
    assert_eq!(args, Vec::<String>::new());

    // Inherit all environment variables and arguments from host

    // Inherit only some environment vars

    // Set custom environment vars and arguments
}
