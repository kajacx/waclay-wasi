use crate::*;

impl<T: WasiP2CtxHolder> crate::bindings::ExitHost for T {
    fn exit(&mut self, status: Result<(), ()>) -> () {
        todo!()
    }
}
