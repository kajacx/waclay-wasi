use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::TerminalStdinHost for T {
    fn get_terminal_stdin(&mut self) -> anyhow::Result<Option<WasiP2TerminalInputResource>> {
        Ok(None)
    }
}
