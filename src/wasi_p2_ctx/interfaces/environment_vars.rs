use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::EnvironmentHost for T {
    fn get_environment(&mut self) -> anyhow::Result<Vec<(String, String)>> {
        Ok(self.as_wasi_mut().environment_vars.clone())
    }

    fn get_arguments(&mut self) -> anyhow::Result<Vec<String>> {
        let mut args_all = vec![self.as_wasi_ref().program_name.clone()];
        args_all.extend_from_slice(&self.as_wasi_ref().cli_arguments);
        Ok(args_all)
    }
}
