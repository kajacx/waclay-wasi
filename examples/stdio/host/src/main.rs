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

    // let interface = instance
    //     .exports()
    //     .instance(&"test:guest/foo".try_into().unwrap())
    //     .unwrap();

    // let select_nth = interface
    //     .func("select-nth")
    //     .unwrap()
    //     .typed::<(Vec<String>, u32), String>()
    //     .unwrap();

    // let example = ["a", "b", "c"]
    //     .iter()
    //     .map(ToString::to_string)
    //     .collect::<Vec<_>>();

    // println!(
    //     "Calling select-nth({example:?}, 1) == {}",
    //     select_nth.call(&mut store, (example.clone(), 1)).unwrap()
    // );
}
