use waclay::*;

mod bindings;

// The bytes of the component.
const WASM: &[u8] =
    include_bytes!("../../guest/target/wasm32-wasip2/debug/example_stdio_guest.wasm");

struct StoreData {
    ctx: waclay_wasi::WasiP2Ctx,
}

impl waclay_wasi::WasiP2CtxHolder for StoreData {
    type Ctx = waclay_wasi::WasiP2Ctx;

    fn get_ctx_mut(&mut self) -> &mut Self::Ctx {
        &mut self.ctx
    }
}

pub fn main() {
    let engine = Engine::new(wasmi_runtime_layer::Engine::default());

    let mut store = Store::new(
        &engine,
        StoreData {
            ctx: waclay_wasi::WasiP2Ctx::new(),
        },
    );

    let component = Component::new(&engine, WASM).unwrap();

    let mut linker = Linker::default();

    waclay_wasi::add_to_linker(&mut linker, &mut store).unwrap();

    let instance = linker.instantiate(&mut store, &component).unwrap();

    let print_stdout = bindings::exports_funcs::get_print_stdout(&instance, &mut store).unwrap();

    print_stdout
        .call(&mut store, "Hello world!".to_string())
        .unwrap();

    let print_stderr = bindings::exports_funcs::get_print_stderr(&instance, &mut store).unwrap();

    print_stderr
        .call(&mut store, "Goodbye world!".to_string())
        .unwrap();

    let read_stdin = bindings::exports_funcs::get_read_stdin(&instance, &mut store).unwrap();

    let result = read_stdin.call(&mut store, ()).unwrap();
    println!("Read from guest: {result}");
    let result = read_stdin.call(&mut store, ()).unwrap();
    println!("Read from guest: {result}");
    let result = read_stdin.call(&mut store, ()).unwrap();
    println!("Read from guest: {result}");
}
