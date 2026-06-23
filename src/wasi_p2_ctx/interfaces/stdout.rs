use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::StdoutHost for T {
    fn get_stdout(&mut self) -> WasiP2OutputStreamResource {
        WasiP2OutputStreamResource::Stdout
    }
}
