use waclay::*;
use waclay_wasi::{AsWasiP2Ctx, WasiP2Ctx};

mod bindings;

// The bytes of the component.
const WASM: &[u8] =
    include_bytes!("../../guest/target/wasm32-wasip2/debug/example_stdio_guest.wasm");

struct StoreData {
    ctx: WasiP2Ctx,
}

impl AsWasiP2Ctx for StoreData {
    fn as_wasi_ctx(&self) -> &WasiP2Ctx {
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

    let print_stdout = bindings::exports_funcs::get_print_stdout(&instance, &mut store).unwrap();
    let print_stderr = bindings::exports_funcs::get_print_stderr(&instance, &mut store).unwrap();
    let read_stdin = bindings::exports_funcs::get_read_stdin(&instance, &mut store).unwrap();

    // Ignoring std io and pretending it's closed is already the default behaviour,
    // but you can call "clear" methods to make sure in case wasi ctx was modified before.
    store.data_mut().as_wasi_mut().clear_stdout().clear_stderr();

    print_stdout
        .call(&mut store, "Voided message to stdout".to_string())
        .unwrap();

    print_stderr
        .call(&mut store, "Voided message to stderr".to_string())
        .unwrap();

    // let result = read_stdin.call(&mut store, ()).unwrap();
    // assert_eq!(result, "");

    // Redirect / inherit stdio from the host
    store
        .data_mut()
        .as_wasi_mut()
        .inherit_stdout()
        .inherit_stderr();

    print_stdout
        .call(&mut store, "Redirected message to stdout".to_string())
        .unwrap();

    print_stderr
        .call(&mut store, "Redirected message to stderr".to_string())
        .unwrap();

    let result = read_stdin.call(&mut store, ()).unwrap();
    assert_eq!(result, "[Rust guest reading stdin]: line1");
    let result = read_stdin.call(&mut store, ()).unwrap();
    assert_eq!(result, "[Rust guest reading stdin]: line2");
    let result = read_stdin.call(&mut store, ()).unwrap();
    assert_eq!(result, "[Rust guest reading stdin]: ");

    // Capture stdio with a custom stream

    store.data_mut().as_wasi_mut().stdout.as_any_mut();
}
