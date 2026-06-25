use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::WallClockHost for T {
    fn now(&mut self) -> bindings::Datetime {
        bindings::Datetime {
            seconds: 0,
            nanoseconds: 0,
        }
    }
}
