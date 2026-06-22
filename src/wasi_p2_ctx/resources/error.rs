use std::sync::LazyLock;

use crate::*;

#[derive(Debug, Clone)]
pub struct WasiP2ErrorResource {}

static RESOURCE_TYPE: LazyLock<ResourceType> = LazyLock::new(|| {
    ResourceType::new::<WasiP2ErrorResource>(Some(TypeIdentifier::new("WasiP2ErrorResource", None)))
});

// static VALUE_TYPE: LazyLock<Re>  = LazyLock::new(|| {
//     ValueType::
// })

impl WasiP2ErrorResource {
    pub fn resource_type() -> ResourceType {
        RESOURCE_TYPE.clone()
    }
}

// impl ComponentType for WasiP2ErrorResource {
//     fn ty() -> ValueType {
//         ValueType::Own(RESOURCE_TYPE.clone())
//     }

//     fn from_value(value: &Value) -> anyhow::Result<Self> {
//         match value {
//             Value::Borrow(borrow) => borrow.rep(ctx)
//         }
//     }

//     fn into_value(self) -> anyhow::Result<Value> {
//         todo!()
//     }
// }
