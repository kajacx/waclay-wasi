use crate::*;

impl<T: WasiP2CtxHolder> bindings::TerminalOutputHost for T {}
