use std::sync::LazyLock;

use crate::*;

#[derive(Debug, Clone)]
pub struct WasiP2PollableResource {}

static RESOURCE_TYPE: LazyLock<ResourceType> = LazyLock::new(|| {
    ResourceType::new::<WasiP2PollableResource>(Some(TypeIdentifier::new(
        "WasiP2PollableResource",
        None,
    )))
});

impl WasiP2PollableResource {
    pub fn resource_type() -> ResourceType {
        RESOURCE_TYPE.clone()
    }
}

impl ResourceConvert for WasiP2PollableResource {
    fn ty_own() -> ValueType {
        ValueType::Own(RESOURCE_TYPE.clone())
    }

    fn ty_borrow() -> ValueType {
        ValueType::Borrow(RESOURCE_TYPE.clone())
    }

    fn from_value(ctx: impl AsContext, value: Value) -> anyhow::Result<Self> {
        match value {
            Value::Own(own) => Ok(own.rep::<Self, _, _>(&ctx.as_context())?.clone()),
            Value::Borrow(borrow) => Ok(borrow.rep::<Self, _, _>(&ctx.as_context())?.clone()),
            _ => anyhow::bail!(
                "Expected resource of type WasiP2TerminalInputResource, but got {value:?} instead"
            ),
        }
    }

    fn to_value(self, ctx: impl AsContextMut) -> anyhow::Result<Value> {
        Ok(Value::Own(ResourceOwn::new(
            ctx,
            self,
            RESOURCE_TYPE.clone(),
        )?))
    }
}
