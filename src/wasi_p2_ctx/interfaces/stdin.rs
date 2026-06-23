use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::StdinHost for T {
    fn get_stdin(&mut self) -> WasiP2InputStreamResource {
        WasiP2InputStreamResource {}
    }
}
