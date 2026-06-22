use crate::*;

impl<T: WasiP2CtxHolder> crate::bindings::TerminalStdoutHost for T {
    fn get_terminal_stdout(&mut self) -> Option<bindings::TerminalOutput> {
        todo!()
    }
}
