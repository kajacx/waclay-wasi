use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::EnvironmentHost for T {
    fn get_environment(&mut self) -> Vec<(String, String)> {
        self.as_wasi_mut().environment_vars.clone()
    }

    fn get_arguments(&mut self) -> Vec<String> {
        let mut args_all = vec![self.as_wasi_ref().program_name.clone()];
        args_all.extend_from_slice(&self.as_wasi_ref().cli_arguments);
        args_all
    }
}
