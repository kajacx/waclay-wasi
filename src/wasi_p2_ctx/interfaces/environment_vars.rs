use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::EnvironmentHost for T {
    fn get_environment(&mut self) -> Vec<(String, String)> {
        self.as_wasi_ctx().environment_vars.clone()
    }
}
