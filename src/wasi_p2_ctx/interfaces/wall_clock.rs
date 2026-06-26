use std::any::Any;

use crate::*;

pub trait WasiP2WallClock: std::fmt::Debug {
    fn now(&mut self) -> bindings::Datetime;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: AsWasiP2Ctx> crate::bindings::WallClockHost for T {
    fn now(&mut self) -> bindings::Datetime {
        self.as_wasi_mut().wall_clock.now()
    }
}

pub(super) mod internal {
    use super::*;
    use std::time::SystemTime;

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub struct EmptyWallClock {}

    impl WasiP2WallClock for EmptyWallClock {
        fn now(&mut self) -> bindings::Datetime {
            bindings::Datetime {
                seconds: 0,
                nanoseconds: 0,
            }
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub struct HostWallClock {}

    impl WasiP2WallClock for HostWallClock {
        fn now(&mut self) -> bindings::Datetime {
            match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(positive) => bindings::Datetime {
                    seconds: positive.as_secs(),
                    nanoseconds: positive.subsec_nanos(),
                },
                Err(negative) => {
                    eprintln!(
                        "Warning! Host OS clock reported time that is before the unix epoch, namely: {:?}",
                        negative.duration()
                    );
                    eprintln!(
                        "Such time cannot be represented in the wasi Datetime structure, returning 0 (Jan 1st, 1970) instead."
                    );
                    bindings::Datetime {
                        seconds: 0,
                        nanoseconds: 0,
                    }
                }
            }
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }
}
