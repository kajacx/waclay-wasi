use crate::*;

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
        }
    }

    pub fn clear_all(&mut self) -> &mut Self {
        self.clear_program_name()
            .clear_cli_arguments()
            .clear_stdin()
            .clear_stdout()
            .clear_stderr()
            .clear_environment_vars()
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
