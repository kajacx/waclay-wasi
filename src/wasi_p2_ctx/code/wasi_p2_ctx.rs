use crate::*;

pub struct WasiP2Ctx {
    pub environment_vars: Vec<(String, String)>,
    pub stdout: Box<dyn WasiP2OutputStream>,
    pub stderr: Box<dyn WasiP2OutputStream>,
}

impl WasiP2Ctx {
    pub fn new() -> Self {
        Self {
            environment_vars: vec![],
            stdout: Box::new(internal::OutputStreamEmpty {}),
            stderr: Box::new(internal::OutputStreamEmpty {}),
        }
    }

    // Environment variables

    pub fn inherit_environment_vars(&mut self) -> &mut Self {
        self.environment_vars = std::env::vars().collect();
        self
    }

    pub fn set_environment_vars<K: Into<String>, V: Into<String>>(
        &mut self,
        vars: impl IntoIterator<Item = (K, V)>,
    ) -> &mut Self {
        let vars: Vec<(String, String)> = vars
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect();

        self.environment_vars = vars;
        self
    }

    pub fn clear_environment_vars(&mut self) -> &mut Self {
        self.environment_vars = vec![];
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
    fn as_wasi_ctx(&mut self) -> &mut WasiP2Ctx;
}
