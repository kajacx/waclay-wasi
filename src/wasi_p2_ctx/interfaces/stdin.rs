use crate::*;

impl<T: WasiP2CtxHolder> crate::bindings::StdinHost for T {
    fn get_stdin(&mut self) -> bindings::InputStream {
        todo!()
    }
}
