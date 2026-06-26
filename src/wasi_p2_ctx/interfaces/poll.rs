use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::PollHost for T {
    fn pollable_block(&mut self, _self_: WasiP2PollableResource) -> anyhow::Result<()> {
        anyhow::bail!("method pollable_block is not yet implemented");
    }
}
