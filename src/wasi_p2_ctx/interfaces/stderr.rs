use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::StderrHost for T {
    fn get_stderr(&mut self) -> anyhow::Result<WasiP2OutputStreamResource> {
        Ok(WasiP2OutputStreamResource::Stderr)
    }
}
