use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::TerminalStderrHost for T {
    fn get_terminal_stderr(&mut self) -> anyhow::Result<Option<WasiP2TerminalOutputResource>> {
        Ok(None)
    }
}
