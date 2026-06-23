use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::TerminalStdinHost for T {
    fn get_terminal_stdin(&mut self) -> Option<WasiP2TerminalInputResource> {
        todo!()
    }
}
