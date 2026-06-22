use waclay::{
    anyhow::{bail, ensure},
    *,
};

mod error;
mod input_stream;
mod output_stream;
mod pollable;
mod terminal_input;
mod terminal_output;

pub use error::*;
pub use input_stream::*;
pub use output_stream::*;
pub use pollable::*;
pub use terminal_input::*;
pub use terminal_output::*;

pub trait ResourceConvert: Sized {
    fn ty_own() -> ValueType;

    fn ty_borrow() -> ValueType;

    fn from_value(ctx: impl AsContext, value: Value) -> anyhow::Result<Self>;

    fn to_value(self, ctx: impl AsContextMut) -> anyhow::Result<Value>;
}

impl<T: ResourceConvert> ResourceConvert for Option<T> {
    fn ty_own() -> ValueType {
        ValueType::Option(OptionType::new(T::ty_own()))
    }

    fn ty_borrow() -> ValueType {
        ValueType::Option(OptionType::new(T::ty_borrow()))
    }

    fn from_value(ctx: impl AsContext, value: Value) -> anyhow::Result<Self> {
        let value_clone = value.clone(); // Rust pls

        if let Value::Option(option) = value.clone() {
            ensure!(
                option.ty().some_ty() == T::ty_own() || option.ty().some_ty() == T::ty_borrow(),
                "Option should be of type {:?} or {:?}, but it is {:?} instead",
                T::ty_own(),
                T::ty_borrow(),
                option.ty().some_ty()
            );

            let option = option.as_ref().map(|v| v);

            if let Some(inner) = option {
                Ok(Some(T::from_value(ctx, inner.clone())?))
            } else {
                Ok(None)
            }
        } else {
            bail!("Expected an option, got {value_clone:?} instead")
        }
    }

    fn to_value(self, ctx: impl AsContextMut) -> anyhow::Result<Value> {
        let value = if let Some(inner) = self {
            Some(inner.to_value(ctx)?)
        } else {
            None
        };

        Ok(Value::Option(OptionValue::new(
            OptionType::new(T::ty_own()),
            value,
        )?))
    }
}
