use crate::*;

impl<T: WasiP2CtxHolder> crate::bindings::StdoutHost for T {
    fn get_stdout(&mut self) -> WasiP2OutputStreamResource {
        todo!()
    }
}
