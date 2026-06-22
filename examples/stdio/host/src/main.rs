use waclay::*;

// The bytes of the component.
const WASM: &[u8] =
    include_bytes!("../../guest/target/wasm32-wasip2/debug/example_stdio_guest.wasm");

pub fn main() {
    let engine = Engine::new(wasmi_runtime_layer::Engine::default());

    let mut store = Store::new(&engine, ());

    let component = Component::new(&engine, WASM).unwrap();

    let linker = Linker::default();

    let instance = linker.instantiate(&mut store, &component).unwrap();

    let interface = instance
        .exports()
        .instance(&"test:guest/foo".try_into().unwrap())
        .unwrap();

    let select_nth = interface
        .func("select-nth")
        .unwrap()
        .typed::<(Vec<String>, u32), String>()
        .unwrap();

    let example = ["a", "b", "c"]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    println!(
        "Calling select-nth({example:?}, 1) == {}",
        select_nth.call(&mut store, (example.clone(), 1)).unwrap()
    );
}
