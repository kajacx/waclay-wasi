use waclay::*;

#[allow(unused)]
mod bindings;

impl ComponentType for bindings::TerminalOutput {
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

impl ComponentType for bindings::InputStream {
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

impl ComponentType for bindings::Error {
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

impl ComponentType for bindings::Pollable {
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

impl ComponentType for bindings::OutputStream {
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

impl ComponentType for bindings::TerminalInput {
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
