use crate::*;

impl<T: WasiP2CtxHolder> crate::bindings::PollHost for T {
    fn pollable_block(&mut self, self_: bindings::Pollable) -> () {
        todo!()
    }
}
