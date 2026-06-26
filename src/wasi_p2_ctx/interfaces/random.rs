use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::RandomHost for T {
    fn get_random_u64(&mut self) -> anyhow::Result<u64> {
        Ok(self.as_wasi_mut().rng.0.next())
    }
}
