use crate::*;

#[derive(Debug, Clone)]
pub struct WasiP2ErrorResource {}

impl WasiP2ErrorResource {
    pub fn resource_type() -> ResourceType {
        todo!()
    }
}

impl ComponentType for WasiP2ErrorResource {
    fn ty() -> ValueType {
        todo!()
    }

    fn from_value(value: &Value) -> anyhow::Result<Self> {
        todo!()
    }

    fn into_value(self) -> anyhow::Result<Value> {
        todo!()
    }
}
