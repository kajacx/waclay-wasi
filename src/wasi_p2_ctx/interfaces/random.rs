use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::RandomHost for T {
    fn get_random_u64(&mut self) -> u64 {
        self.as_wasi_mut().rng.next()
    }
}
