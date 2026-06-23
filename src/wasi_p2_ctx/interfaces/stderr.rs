use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::StderrHost for T {
    fn get_stderr(&mut self) -> WasiP2OutputStreamResource {
        WasiP2OutputStreamResource::Stderr
    }
}
