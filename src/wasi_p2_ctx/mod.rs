mod environment_vars;

pub use environment_vars::*;

pub struct WasiP2Ctx {
    environment_vars: Box<dyn WasiP2EnvironmentVars>,
}

impl WasiP2Ctx {
    pub fn new() -> Self {
        Self {
            environment_vars: Box::new(WasiP2EnvironmentVarsConst::empty()),
        }
    }

    pub fn clear_environment_vars(&mut self) -> &mut Self {
        self.environment_vars = Box::new(WasiP2EnvironmentVarsConst::empty());
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

        self.environment_vars = Box::new(WasiP2EnvironmentVarsConst { vars });
        self
    }

    pub fn set_environment_vars_custom(
        &mut self,
        vars: Box<dyn WasiP2EnvironmentVars>,
    ) -> &mut Self {
        self.environment_vars = vars;
        self
    }
}

pub trait WasiP2CtxTrait {
    fn get_environment_vars(&mut self) -> Vec<(String, String)>;
}

impl WasiP2CtxTrait for WasiP2Ctx {
    fn get_environment_vars(&mut self) -> Vec<(String, String)> {
        self.environment_vars.get_environment_vars()
    }
}

pub trait WasiP2CtxHolder {
    type Ctx: WasiP2CtxTrait;

    fn get_ctx_mut(&mut self) -> &mut Self::Ctx;
}
