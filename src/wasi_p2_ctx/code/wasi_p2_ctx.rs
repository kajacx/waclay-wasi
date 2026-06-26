use crate::*;
use simple_rng::*;

#[derive(Debug)]
pub struct WasiP2Ctx {
    pub environment_vars: Vec<(String, String)>,

    /// Cli arguments and program name are separated
    pub program_name: String,

    /// Cli arguments and program name are separated, this list will *not* contain the program name,
    /// as that is stored in a separate `program_name` field.
    pub cli_arguments: Vec<String>,

    pub stdin: Box<dyn WasiP2InputStream>,
    pub stdout: Box<dyn WasiP2OutputStream>,
    pub stderr: Box<dyn WasiP2OutputStream>,

    pub rng: internal::DebugRng,

    pub wall_clock: Box<dyn WasiP2WallClock>,

    /// Caution!
    ///
    /// Changing monotonic clock implementation while a component instance is active can lead to unexpected
    /// shifts in the monotonic clock from the instance's perspective, even going backwards!
    ///
    /// The way it works is that there is a single host function "monotonic_clock:now()" that returns
    /// the amount of nanoseconds passed since some set point in time.
    ///
    /// The component then grabs this number and stores it as an `instant`. It then later calls the host function again,
    /// subtracts the previously received instance, and voila, component now has `duration` elapsed since that `instant`.
    ///
    /// But what happens when we change the `monotonic_clock` implementation in-between? For example, from measuring time
    /// from 10:00 to 11:00? Let's follow a hypothetical timeline:
    ///
    /// - `.inherit_monotonic_clock()` is called at 10:00, which just stores that timestamp and then returns nanoseconds since that time.
    /// - The component requests and `instant` at 10:40, getting 40 minutes worth if nanoseconds back.
    /// - `.inherit_monotonic_clock()` is called *again* at 11:00, meaning the host will now start returning difference from that time instead.
    /// - The component asks how much time has passed since the earlier obtained `instance` at 11:20.
    ///
    /// At this point, the wasi runtime will say that 20 minutes have passed from the newly set point in time (11:00), even though
    /// earlier the monotonic clock said that 40 minutes has passed. It went back in time!
    ///
    /// **TL;DR:** Do not touch this while an instance is running unless you really know what you are doing.
    pub monotonic_clock: Box<dyn WasiP2MonotonicClock>,
}

impl WasiP2Ctx {
    pub fn new() -> Self {
        Self {
            environment_vars: vec![],
            program_name: "".to_string(),
            cli_arguments: vec![],
            stdin: Box::new(internal::InputStreamEmpty {}),
            stdout: Box::new(internal::OutputStreamEmpty {}),
            stderr: Box::new(internal::OutputStreamEmpty {}),
            rng: internal::DebugRng(RNG::new(DEFAULT_SEED)),
            wall_clock: Box::new(internal::EmptyWallClock {}),
            monotonic_clock: Box::new(internal::EmptyMonotonicClock {}),
        }
    }

    pub fn clear_all(&mut self) -> &mut Self {
        self.clear_program_name()
            .clear_cli_arguments()
            .clear_stdin()
            .clear_stdout()
            .clear_stderr()
            .clear_environment_vars()
            .clear_rng()
            .clear_wall_clock()
            .clear_monotonic_clock()
    }

    // Environment variables

    /// Inherits specified environment variables.
    ///
    /// **Does not clear previously set variables!** To do that, call `clear_environment_vars` beforehand.
    pub fn inherit_environment_vars(
        &mut self,
        vars: impl IntoIterator<Item = impl AsRef<str>>,
        mode: InsertionMode,
    ) -> &mut Self {
        if mode == InsertionMode::RemovePrevious {
            self.environment_vars = vars
                .into_iter()
                .filter_map(|name| {
                    std::env::var(name.as_ref())
                        .ok()
                        .map(|value| (name.as_ref().to_string(), value))
                })
                .collect();
        } else {
            vars.into_iter().for_each(|name| {
                if let Ok(value) = std::env::var(name.as_ref()) {
                    self.set_env_var(
                        name.as_ref().to_string(),
                        value,
                        mode == InsertionMode::Merge,
                    );
                }
            });
        }
        self
    }

    /// Sets provided environment variables to specified values.
    pub fn set_environment_vars<K: Into<String>, V: Into<String>>(
        &mut self,
        vars: impl IntoIterator<Item = (K, V)>,
        mode: InsertionMode,
    ) -> &mut Self {
        if mode == InsertionMode::RemovePrevious {
            self.environment_vars = vars
                .into_iter()
                .map(|(name, value)| (name.into(), value.into()))
                .collect();
        } else {
            vars.into_iter().for_each(|(name, value)| {
                self.set_env_var(name.into(), value.into(), mode == InsertionMode::Merge)
            });
        }
        self
    }

    pub fn clear_environment_vars(&mut self) -> &mut Self {
        self.environment_vars = vec![];
        self
    }

    fn set_env_var(&mut self, name: String, value: String, replace_old: bool) {
        let existing = self.environment_vars.iter_mut().find(|(n, _)| n == &name);
        if let Some(existing) = existing {
            if replace_old {
                existing.1 = value;
            }
        } else {
            self.environment_vars.push((name, value));
        }
    }

    // Program name

    pub fn inherit_program_name(&mut self) -> &mut Self {
        self.program_name = std::env::args().next().unwrap_or_default();
        self
    }

    pub fn set_program_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.program_name = name.into();
        self
    }

    pub fn clear_program_name(&mut self) -> &mut Self {
        self.program_name = "".to_string();
        self
    }

    // Cli arguments

    /// Cli arguments are separated from program name.
    /// To also inherit program name, call `inherit_program_name`.
    pub fn inherit_cli_arguments(&mut self) -> &mut Self {
        self.cli_arguments = std::env::args().skip(1).collect();
        self
    }

    /// Cli arguments are separated from program name.
    /// To also set program name, call `set_program_name`.
    ///
    /// If you already have both program name and cli arguments in a single list, you can call `set_cli_arguments(list.skip(1))`
    pub fn set_cli_arguments(
        &mut self,
        args: impl IntoIterator<Item = impl Into<String>>,
    ) -> &mut Self {
        self.cli_arguments = args.into_iter().map(|item| item.into()).collect();
        self
    }

    /// Cli arguments are separated from program name.
    /// To also clear program name, call `clear_program_name`.
    pub fn clear_cli_arguments(&mut self) -> &mut Self {
        self.cli_arguments = vec![];
        self
    }

    // Stdin

    pub fn inherit_stdin(&mut self) -> &mut Self {
        self.stdin = Box::new(internal::InputStreamInherit {});
        self
    }

    pub fn set_stdin(&mut self, stdin: Box<dyn WasiP2InputStream>) -> &mut Self {
        self.stdin = stdin;
        self
    }

    pub fn clear_stdin(&mut self) -> &mut Self {
        self.stdin = Box::new(internal::InputStreamEmpty {});
        self
    }

    // Stdout

    pub fn inherit_stdout(&mut self) -> &mut Self {
        self.stdout = Box::new(internal::OutputStreamStdout {});
        self
    }

    pub fn set_stdout(&mut self, stdout: Box<dyn WasiP2OutputStream>) -> &mut Self {
        self.stdout = stdout;
        self
    }

    pub fn clear_stdout(&mut self) -> &mut Self {
        self.stdout = Box::new(internal::OutputStreamEmpty {});
        self
    }

    // Stderr

    pub fn inherit_stderr(&mut self) -> &mut Self {
        self.stderr = Box::new(internal::OutputStreamStderr {});
        self
    }

    pub fn set_stderr(&mut self, stderr: Box<dyn WasiP2OutputStream>) -> &mut Self {
        self.stderr = stderr;
        self
    }

    pub fn clear_stderr(&mut self) -> &mut Self {
        self.stderr = Box::new(internal::OutputStreamEmpty {});
        self
    }

    // Rng

    /// This doesn't redirect every RNG request to the host/OS.
    /// It simply resets the RNG generator with a "genuinely" random seed.
    ///
    /// This returns an error if the underlying "os level" function fails.
    pub fn inherit_rng(&mut self) -> Result<&mut Self, getrandom::Error> {
        self.rng = internal::DebugRng(RNG::new(getrandom::u64()?));
        Ok(self)
    }

    pub fn set_rng(&mut self, seed: u64) -> &mut Self {
        self.rng = internal::DebugRng(RNG::new(seed));
        self
    }

    /// Resets the rng generator with the default seed.
    pub fn clear_rng(&mut self) -> &mut Self {
        self.rng = internal::DebugRng(RNG::new(DEFAULT_SEED));
        self
    }

    // Wall clock

    pub fn inherit_wall_clock(&mut self) -> &mut Self {
        self.wall_clock = Box::new(internal::HostWallClock {});
        self
    }

    pub fn set_wall_clock(&mut self, wall_clock: Box<dyn WasiP2WallClock>) -> &mut Self {
        self.wall_clock = wall_clock;
        self
    }

    pub fn clear_wall_clock(&mut self) -> &mut Self {
        self.wall_clock = Box::new(internal::EmptyWallClock {});
        self
    }

    // Monotonic clock

    /// Caution!
    ///
    /// Changing monotonic clock implementation while a component instance is active can lead to unexpected
    /// shifts in the monotonic clock from the instance's perspective, even going backwards!
    ///
    /// See the `monotonic_clock` field for more detail.
    pub fn inherit_monotonic_clock(&mut self) -> &mut Self {
        self.monotonic_clock = Box::new(internal::HostMonotonicClock {
            start: std::time::Instant::now(),
        });
        self
    }

    /// Caution!
    ///
    /// Changing monotonic clock implementation while a component instance is active can lead to unexpected
    /// shifts in the monotonic clock from the instance's perspective, even going backwards!
    ///
    /// See the `monotonic_clock` field for more detail.
    pub fn set_monotonic_clock(
        &mut self,
        monotonic_clock: Box<dyn WasiP2MonotonicClock>,
    ) -> &mut Self {
        self.monotonic_clock = monotonic_clock;
        self
    }

    /// Caution!
    ///
    /// Changing monotonic clock implementation while a component instance is active can lead to unexpected
    /// shifts in the monotonic clock from the instance's perspective, even going backwards!
    ///
    /// See the `monotonic_clock` field for more detail.
    pub fn clear_monotonic_clock(&mut self) -> &mut Self {
        self.monotonic_clock = Box::new(internal::EmptyMonotonicClock {});
        self
    }
}

pub trait AsWasiP2Ctx {
    fn as_wasi_ref(&self) -> &WasiP2Ctx;

    fn as_wasi_mut(&mut self) -> &mut WasiP2Ctx;
}

impl AsWasiP2Ctx for WasiP2Ctx {
    fn as_wasi_ref(&self) -> &WasiP2Ctx {
        self
    }

    fn as_wasi_mut(&mut self) -> &mut WasiP2Ctx {
        self
    }
}

/// Determines whether or not to remove previous values when inserting new ones.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InsertionMode {
    /// Remove all previous values before appending new ones.
    RemovePrevious,

    /// Insert all new values, replacing only conflicting values. Default behaviour.
    Merge,

    /// Insert only new values, skipping over conflicting ones.
    AppendOnly,
}

impl Default for InsertionMode {
    fn default() -> Self {
        Self::Merge
    }
}

const DEFAULT_SEED: u64 = 3_141_592_653_589_793_238u64;
