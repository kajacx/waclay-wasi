use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::PreopensHost for T {
    fn get_directories(&mut self) -> anyhow::Result<Vec<(WasiP2DescriptorResource, String)>> {
        Ok(vec![])
    }
}
