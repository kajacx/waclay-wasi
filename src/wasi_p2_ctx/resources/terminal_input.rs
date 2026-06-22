use crate::*;

#[derive(Debug, Clone)]
pub struct WasiP2TerminalInputResource {}

impl WasiP2TerminalInputResource {
    pub fn resource_type() -> ResourceType {
        todo!()
    }
}

// impl ComponentType for WasiP2TerminalInputResource {
//     fn ty() -> ValueType {
//         todo!()
//     }

//     fn from_value(value: &Value) -> anyhow::Result<Self> {
//         todo!()
//     }

//     fn into_value(self) -> anyhow::Result<Value> {
//         todo!()
//     }
// }
