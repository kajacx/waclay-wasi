use crate::*;

mod environment_vars;
mod error;
mod exit;
mod monotonic_clock;
mod poll;
mod preopens;
mod random;
mod stderr;
mod stdin;
mod stdout;
mod streams;
mod terminal_input;
mod terminal_output;
mod terminal_stderr;
mod terminal_stdin;
mod terminal_stdout;
mod types;
mod wall_clock;

pub use monotonic_clock::*;
pub use streams::*;
pub use wall_clock::*;

pub fn add_to_linker<S: AsWasiP2Ctx + 'static, E: wasm_runtime_layer::backend::WasmEngine>(
    linker: &mut Linker,
    store: &mut Store<S, E>,
) -> anyhow::Result<()> {
    bindings::imports::register_environment_host(linker, store)?;
    bindings::imports::register_error_host(linker, store)?;
    bindings::imports::register_exit_host(linker, store)?;
    bindings::imports::register_poll_host(linker, store)?;
    bindings::imports::register_preopens_host(linker, store)?;
    bindings::imports::register_random_host(linker, store)?;
    bindings::imports::register_stdin_host(linker, store)?;
    bindings::imports::register_stdout_host(linker, store)?;
    bindings::imports::register_stderr_host(linker, store)?;
    bindings::imports::register_streams_host(linker, store)?;
    bindings::imports::register_terminal_output_host(linker, store)?;
    bindings::imports::register_terminal_input_host(linker, store)?;
    bindings::imports::register_terminal_stdin_host(linker, store)?;
    bindings::imports::register_terminal_stdout_host(linker, store)?;
    bindings::imports::register_terminal_stderr_host(linker, store)?;
    bindings::imports::register_types_host(linker, store)?;
    bindings::imports::register_monotonic_clock_host(linker, store)?;
    bindings::imports::register_wall_clock_host(linker, store)?;
    Ok(())
}

pub(super) mod internal {
    pub use super::monotonic_clock::internal::*;
    pub use super::streams::internal::*;
    pub use super::wall_clock::internal::*;
}
