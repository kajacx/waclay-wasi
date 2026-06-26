use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::StdinHost for T {
    fn get_stdin(&mut self) -> anyhow::Result<WasiP2InputStreamResource> {
        Ok(WasiP2InputStreamResource {})
    }
}
