use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::MonotonicClockHost for T {
    fn now(&mut self) -> bindings::Instant {
        0
    }
}
