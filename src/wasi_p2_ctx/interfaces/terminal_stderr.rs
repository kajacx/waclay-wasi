use crate::*;

impl<T: WasiP2CtxHolder> crate::bindings::TerminalStderrHost for T {
    fn get_terminal_stderr(&mut self) -> Option<WasiP2TerminalOutputResource> {
        todo!()
    }
}
