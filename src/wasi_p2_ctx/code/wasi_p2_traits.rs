use crate::*;

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
