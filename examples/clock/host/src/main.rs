use std::time::{Duration, SystemTime};

use waclay::*;
use waclay_wasi::{AsWasiP2Ctx, WasiP2Ctx, WasiP2MonotonicClock, WasiP2WallClock};

mod bindings;

// The bytes of the component.
const WASM: &[u8] =
    include_bytes!("../../guest/target/wasm32-wasip2/debug/example_clock_guest.wasm");

struct StoreData {
    ctx: WasiP2Ctx,
}

impl AsWasiP2Ctx for StoreData {
    fn as_wasi(&self) -> &WasiP2Ctx {
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

    // Inherit clock from the host
    // We will only test that the functions return withing a minute of their execution

    store
        .data_mut()
        .as_wasi_mut()
        .clear_all()
        .inherit_wall_clock()
        .inherit_monotonic_clock();
    // Warning! You DO NOT want to change monotonic clock impl while an component is active.
    // It is OK here only because we know that the component does not hold any `Instant` values.

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let component_time = get_wall_clock.call(&mut store, ()).unwrap() as i64;
    let diff = component_time - current_time;
    assert!(
        diff >= 0 && diff < 60,
        "Component method should have returned a value within a minute of it being called, diff was: {diff}."
    );

    let tile_elapsed = get_time_elapsed.call(&mut store, ()).unwrap();
    assert!(
        tile_elapsed < 60_000_000_000, // One minute
        "Component call should have taken less than a minute, it took {tile_elapsed} nano seconds instead."
    );

    // Custom clock implementation

    let custom_time = SystemTime::UNIX_EPOCH + Duration::from_hours(24);

    store
        .data_mut()
        .as_wasi_mut()
        .clear_all()
        .set_wall_clock(Box::new(CustomWallClock { time: custom_time }))
        .set_monotonic_clock(Box::new(CustomMonotonicClock { time_passed: 0 }));
    // Warning! You DO NOT want to change monotonic clock impl while an component is active.
    // It is OK here only because we know that the component does not hold any `Instant` values.

    let component_time_seconds = get_wall_clock.call(&mut store, ()).unwrap();
    let component_time = SystemTime::UNIX_EPOCH + Duration::from_secs(component_time_seconds);
    assert_eq!(component_time, custom_time);

    let time_elapsed = get_time_elapsed.call(&mut store, ()).unwrap();
    assert_eq!(time_elapsed, 5_000_000_000)
}

#[derive(Debug)]
struct CustomWallClock {
    time: SystemTime,
}

impl WasiP2WallClock for CustomWallClock {
    fn now(&mut self) -> waclay_wasi::bindings::Datetime {
        let elapsed = self.time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        waclay_wasi::bindings::Datetime {
            seconds: elapsed.as_secs(),
            nanoseconds: elapsed.subsec_nanos(),
        }
    }
}

#[derive(Debug)]
struct CustomMonotonicClock {
    time_passed: u64,
}

impl WasiP2MonotonicClock for CustomMonotonicClock {
    fn now(&mut self) -> waclay_wasi::bindings::Instant {
        let result = self.time_passed;
        self.time_passed += 5_000_000_000; // Simulate 5 seconds passing between every call
        result
    }
}
