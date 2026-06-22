use crate::*;

impl<T: WasiP2CtxHolder> crate::bindings::TerminalStdinHost for T {
    fn get_terminal_stdin(&mut self) -> Option<bindings::TerminalInput> {
        todo!()
    }
}
