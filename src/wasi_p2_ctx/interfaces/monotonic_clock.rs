use std::any::Any;

use crate::*;

pub trait WasiP2MonotonicClock: std::fmt::Debug {
    fn now(&mut self) -> bindings::Instant;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: AsWasiP2Ctx> crate::bindings::MonotonicClockHost for T {
    fn now(&mut self) -> bindings::Instant {
        self.as_wasi_mut().monotonic_clock.now()
    }
}

pub(super) mod internal {
    use super::*;

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub struct EmptyMonotonicClock {}

    impl WasiP2MonotonicClock for EmptyMonotonicClock {
        fn now(&mut self) -> bindings::Instant {
            0
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub struct HostMonotonicClock {
        /// Caution!
        ///
        /// Changing the start instant while a component instance is active can lead to unexpected
        /// shifts in the monotonic clock from the instance's perspective, even going backwards!
        ///
        /// See the `monotonic_clock` field in `WasiP2Ctx` for more detail.
        pub start: std::time::Instant,
    }

    impl WasiP2MonotonicClock for HostMonotonicClock {
        fn now(&mut self) -> bindings::Instant {
            self.start.elapsed().as_nanos() as u64
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }
}
