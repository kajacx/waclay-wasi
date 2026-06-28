use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::RandomHost for T {
    fn get_random_u64(&mut self) -> anyhow::Result<u64> {
        Ok(self.as_wasi_mut().rng.0.next())
    }

    fn get_random_bytes(&mut self, len: u64) -> anyhow::Result<Vec<u8>> {
        let len = len as usize;
        let rng = &mut self.as_wasi_mut().rng.0;

        let mut bytes = vec![0u8; len];
        let mut index = 0;
        let mut value = 0;

        while index < len {
            if index % 8 == 0 {
                value = rng.next();
            }

            bytes[index] = value as u8;
            value = value >> 8;
            index = index + 1;
        }

        Ok(bytes)
    }
}
