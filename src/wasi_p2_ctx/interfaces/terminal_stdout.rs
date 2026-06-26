use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::TerminalStdoutHost for T {
    fn get_terminal_stdout(&mut self) -> anyhow::Result<Option<WasiP2TerminalOutputResource>> {
        Ok(None)
    }
}
