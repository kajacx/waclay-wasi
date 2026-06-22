use crate::{WasiP2CtxHolder, WasiP2CtxTrait};

pub trait WasiP2EnvironmentVars {
    fn get_environment_vars(&mut self) -> Vec<(String, String)>;
}

impl<T: WasiP2CtxHolder> crate::bindings::EnvironmentHost for T {
    fn get_environment(&mut self) -> Vec<(String, String)> {
        self.get_ctx_mut().get_environment_vars()
    }
}

pub struct WasiP2EnvironmentVarsConst {
    pub vars: Vec<(String, String)>,
}

impl WasiP2EnvironmentVarsConst {
    pub fn empty() -> Self {
        Self { vars: vec![] }
    }
}

impl WasiP2EnvironmentVars for WasiP2EnvironmentVarsConst {
    fn get_environment_vars(&mut self) -> Vec<(String, String)> {
        self.vars.clone()
    }
}
