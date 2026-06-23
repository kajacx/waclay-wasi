use crate::*;

impl<T: WasiP2CtxHolder> crate::bindings::StderrHost for T {
    fn get_stderr(&mut self) -> WasiP2OutputStreamResource {
        WasiP2OutputStreamResource::Stderr
    }
}
