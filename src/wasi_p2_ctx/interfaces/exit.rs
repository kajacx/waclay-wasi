use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::ExitHost for T {
    fn exit(&mut self, status: Result<(), ()>) -> anyhow::Result<()> {
        let status = status.map_or_else(|_| "Ok", |_| "Err");
        anyhow::bail!("Component called 'exit' function with status {}", status);
    }
}
