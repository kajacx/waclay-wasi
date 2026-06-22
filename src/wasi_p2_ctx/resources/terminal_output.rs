use crate::*;

#[derive(Debug, Clone)]
pub struct WasiP2TerminalOutputResource {}

impl WasiP2TerminalOutputResource {
    pub fn resource_type() -> ResourceType {
        todo!()
    }
}

// impl ComponentType for WasiP2TerminalOutputResource {
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
