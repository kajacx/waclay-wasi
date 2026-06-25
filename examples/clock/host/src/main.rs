use waclay::*;
use waclay_wasi::{AsWasiP2Ctx, WasiP2Ctx};

mod bindings;

// The bytes of the component.
const WASM: &[u8] =
    include_bytes!("../../guest/target/wasm32-wasip2/debug/example_clock_guest.wasm");

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
    let instance = linker.instantiate(&mut store, &component).unwrap();

    let get_wall_clock =
        bindings::exports_funcs::get_get_wall_clock(&instance, &mut store).unwrap();
    let get_time_elapsed =
        bindings::exports_funcs::get_get_time_elapsed(&instance, &mut store).unwrap();

    // Clocks just say 0 by default

    let wall_clock = get_wall_clock.call(&mut store, ()).unwrap();
    assert_eq!(wall_clock, 0);

    let time_elapsed = get_time_elapsed.call(&mut store, ()).unwrap();
    assert_eq!(time_elapsed, 0);
}
