mod pollable;
pub use pollable::*;

pub trait WasiP2Resources {
    type Pollable: WasiP2Pollable;
}
