use std::sync::LazyLock;

use crate::*;

#[derive(Debug, Clone)]
pub struct WasiP2OutputStreamResource {}

static RESOURCE_TYPE: LazyLock<ResourceType> = LazyLock::new(|| {
    ResourceType::new::<WasiP2OutputStreamResource>(Some(TypeIdentifier::new(
        "WasiP2OutputStreamResource",
        None,
    )))
});

impl WasiP2OutputStreamResource {
    pub fn resource_type() -> ResourceType {
        RESOURCE_TYPE.clone()
    }
}

impl ComponentType for WasiP2OutputStreamResource {
    fn ty() -> ValueType {
        ValueType::Own(RESOURCE_TYPE.clone())
    }

    fn from_value(value: &Value, ctx: impl AsContext) -> anyhow::Result<Self> {
        match value {
            Value::Own(own) => Ok(own.rep::<Self, _, _>(&ctx.as_context())?.clone()),
            Value::Borrow(borrow) => Ok(borrow.rep::<Self, _, _>(&ctx.as_context())?.clone()),
            _ => anyhow::bail!(
                "Expected resource of type WasiP2OutputStreamResource, but got {value:?} instead"
            ),
        }
    }

    fn into_value(self, ctx: impl AsContextMut) -> anyhow::Result<Value> {
        Ok(Value::Own(ResourceOwn::new(
            ctx,
            self,
            RESOURCE_TYPE.clone(),
        )?))
    }
}
