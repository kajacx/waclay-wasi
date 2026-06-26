use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::StdoutHost for T {
    fn get_stdout(&mut self) -> anyhow::Result<WasiP2OutputStreamResource> {
        Ok(WasiP2OutputStreamResource::Stdout)
    }
}
