use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::ExitHost for T {
    fn exit(&mut self, _status: Result<(), ()>) -> () {
        todo!()
    }
}
