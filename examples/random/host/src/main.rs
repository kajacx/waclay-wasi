use waclay::*;
use waclay_wasi::{AsWasiP2Ctx, WasiP2Ctx};

mod bindings;

// The bytes of the component.
const WASM: &[u8] =
    include_bytes!("../../guest/target/wasm32-wasip2/debug/example_random_guest.wasm");

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
    // Prepare store, component and exports.

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

    let get_random_bytes =
        bindings::exports_funcs::get_get_random_bytes(&instance, &mut store).unwrap();

    // Default rng should always be the same.

    let default_bytes1 = get_random_bytes.call(&mut store, 16).unwrap();
    assert_eq!(
        default_bytes1,
        vec![
            159, 36, 6, 247, 148, 73, 236, 28, 244, 80, 202, 252, 32, 107, 94, 224
        ]
    );

    let default_bytes2 = get_random_bytes.call(&mut store, 16).unwrap();
    assert_eq!(
        default_bytes2,
        vec![
            229, 70, 189, 91, 226, 46, 234, 46, 66, 17, 186, 69, 196, 217, 180, 151
        ]
    );

    let default_bytes3 = get_random_bytes.call(&mut store, 16).unwrap();
    assert_eq!(
        default_bytes2,
        vec![
            229, 70, 189, 91, 226, 46, 234, 46, 66, 17, 186, 69, 196, 217, 180, 151
        ]
    );

    // Generate rng seed from the host's rng.
    // We will only test that these bytes are different from the default ones and each other.

    store
        .data_mut()
        .as_wasi_mut()
        .clear_all()
        .inherit_rng()
        .unwrap();

    let inherit_bytes1 = get_random_bytes.call(&mut store, 16).unwrap();
    assert_ne!(inherit_bytes1, default_bytes1);

    let inherit_bytes2 = get_random_bytes.call(&mut store, 16).unwrap();
    assert_ne!(inherit_bytes2, default_bytes2);
    assert_ne!(inherit_bytes2, inherit_bytes1);

    let inherit_bytes3 = get_random_bytes.call(&mut store, 16).unwrap();
    assert_ne!(inherit_bytes3, default_bytes3);
    assert_ne!(inherit_bytes3, inherit_bytes1);
    assert_ne!(inherit_bytes3, inherit_bytes2);

    // Set custom environment vars and arguments.

    store.data_mut().as_wasi_mut().clear_all().set_rng(67);

    let bytes = get_random_bytes.call(&mut store, 16).unwrap();
    assert_eq!(
        bytes,
        vec![
            200, 72, 32, 11, 219, 231, 114, 29, 41, 3, 48, 181, 147, 230, 86, 169
        ]
    );

    let bytes = get_random_bytes.call(&mut store, 16).unwrap();
    assert_eq!(
        bytes,
        vec![
            54, 229, 222, 173, 240, 38, 217, 162, 127, 20, 82, 148, 213, 95, 236, 144
        ]
    );

    let bytes = get_random_bytes.call(&mut store, 16).unwrap();
    assert_eq!(
        bytes,
        vec![
            84, 155, 131, 108, 78, 37, 149, 94, 197, 249, 20, 181, 24, 15, 88, 37
        ]
    );
}
