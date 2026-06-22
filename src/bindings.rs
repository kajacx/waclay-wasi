// AUTO-GENERATED WIT BINDINGS for wasm-component-layer
// DO NOT EDIT - Regenerate using wit-bindgen-wcl

#![allow(dead_code, unused_imports, ambiguous_glob_reexports)]

use anyhow::*;
use waclay::*;
use wasm_runtime_layer::backend;

// ========== Type Definitions ==========

/// Resource type: Pollable
///
/// This is a host-managed resource. You should define this type
/// with your actual resource data, then use the manual registration
/// pattern shown below instead of the generated trait.
///
/// Example:
/// ```rust,ignore
/// #[derive(Debug)]
/// pub struct Pollable {
///     // Your resource data here
///     value: i32,
/// }
///
/// // Manual registration (replaces generated trait):
/// let resource_ty = ResourceType::new::<Pollable>(None);
///
/// // Constructor:
/// interface.define_func("[constructor]pollable",
///     Func::new(&mut store,
///         FuncType::new([/* params */], [ValueType::Own(resource_ty.clone())]),
///         move |ctx, args, results| {
///             // Extract params and create resource
///             results[0] = Value::Own(ResourceOwn::new(
///                 ctx,
///                 Pollable(/* data */),
///                 resource_ty.clone(),
///             )?);
///             Ok(())
///         }
///     )
/// )?;
///
/// // Methods:
/// interface.define_func("[method]pollable.method-name",
///     Func::new(&mut store,
///         FuncType::new([ValueType::Borrow(resource_ty.clone())], []),
///         |ctx, args, _| {
///             let Value::Borrow(res) = &args[0] else {
///                 bail!("Expected Borrow")
///             };
///             let data = res.rep::<Pollable, _, _>(&ctx.as_context())?;
///             // Use data
///             Ok(())
///         }
///     )
/// )?;
/// ```
///
/// See the waclay examples/resource.rs for a complete working example.
#[derive(Debug)]
pub struct Pollable {
    // TODO: Add your resource fields
    _placeholder: (),
}

impl Pollable {
    /// Get the ResourceType for this resource.
    ///
    /// This helper method creates a ResourceType for use in manual
    /// resource registration. See the documentation above for usage.
    pub fn resource_type() -> ResourceType {
        ResourceType::new::<Self>(Some(TypeIdentifier::new("Pollable pls", None)))
    }
}

/// Resource type: Error
///
/// This is a host-managed resource. You should define this type
/// with your actual resource data, then use the manual registration
/// pattern shown below instead of the generated trait.
///
/// Example:
/// ```rust,ignore
/// #[derive(Debug)]
/// pub struct Error {
///     // Your resource data here
///     value: i32,
/// }
///
/// // Manual registration (replaces generated trait):
/// let resource_ty = ResourceType::new::<Error>(None);
///
/// // Constructor:
/// interface.define_func("[constructor]error",
///     Func::new(&mut store,
///         FuncType::new([/* params */], [ValueType::Own(resource_ty.clone())]),
///         move |ctx, args, results| {
///             // Extract params and create resource
///             results[0] = Value::Own(ResourceOwn::new(
///                 ctx,
///                 Error(/* data */),
///                 resource_ty.clone(),
///             )?);
///             Ok(())
///         }
///     )
/// )?;
///
/// // Methods:
/// interface.define_func("[method]error.method-name",
///     Func::new(&mut store,
///         FuncType::new([ValueType::Borrow(resource_ty.clone())], []),
///         |ctx, args, _| {
///             let Value::Borrow(res) = &args[0] else {
///                 bail!("Expected Borrow")
///             };
///             let data = res.rep::<Error, _, _>(&ctx.as_context())?;
///             // Use data
///             Ok(())
///         }
///     )
/// )?;
/// ```
///
/// See the waclay examples/resource.rs for a complete working example.
#[derive(Debug, Clone)]
pub struct Error {
    // TODO: Add your resource fields
    _placeholder: (),
}

impl Error {
    /// Get the ResourceType for this resource.
    ///
    /// This helper method creates a ResourceType for use in manual
    /// resource registration. See the documentation above for usage.
    pub fn resource_type() -> ResourceType {
        ResourceType::new::<Self>(None)
    }
}

/// Resource type: InputStream
///
/// This is a host-managed resource. You should define this type
/// with your actual resource data, then use the manual registration
/// pattern shown below instead of the generated trait.
///
/// Example:
/// ```rust,ignore
/// #[derive(Debug)]
/// pub struct InputStream {
///     // Your resource data here
///     value: i32,
/// }
///
/// // Manual registration (replaces generated trait):
/// let resource_ty = ResourceType::new::<InputStream>(None);
///
/// // Constructor:
/// interface.define_func("[constructor]input_stream",
///     Func::new(&mut store,
///         FuncType::new([/* params */], [ValueType::Own(resource_ty.clone())]),
///         move |ctx, args, results| {
///             // Extract params and create resource
///             results[0] = Value::Own(ResourceOwn::new(
///                 ctx,
///                 InputStream(/* data */),
///                 resource_ty.clone(),
///             )?);
///             Ok(())
///         }
///     )
/// )?;
///
/// // Methods:
/// interface.define_func("[method]input_stream.method-name",
///     Func::new(&mut store,
///         FuncType::new([ValueType::Borrow(resource_ty.clone())], []),
///         |ctx, args, _| {
///             let Value::Borrow(res) = &args[0] else {
///                 bail!("Expected Borrow")
///             };
///             let data = res.rep::<InputStream, _, _>(&ctx.as_context())?;
///             // Use data
///             Ok(())
///         }
///     )
/// )?;
/// ```
///
/// See the waclay examples/resource.rs for a complete working example.
#[derive(Debug)]
pub struct InputStream {
    // TODO: Add your resource fields
    _placeholder: (),
}

impl InputStream {
    /// Get the ResourceType for this resource.
    ///
    /// This helper method creates a ResourceType for use in manual
    /// resource registration. See the documentation above for usage.
    pub fn resource_type() -> ResourceType {
        ResourceType::new::<Self>(None)
    }
}

/// Resource type: OutputStream
///
/// This is a host-managed resource. You should define this type
/// with your actual resource data, then use the manual registration
/// pattern shown below instead of the generated trait.
///
/// Example:
/// ```rust,ignore
/// #[derive(Debug)]
/// pub struct OutputStream {
///     // Your resource data here
///     value: i32,
/// }
///
/// // Manual registration (replaces generated trait):
/// let resource_ty = ResourceType::new::<OutputStream>(None);
///
/// // Constructor:
/// interface.define_func("[constructor]output_stream",
///     Func::new(&mut store,
///         FuncType::new([/* params */], [ValueType::Own(resource_ty.clone())]),
///         move |ctx, args, results| {
///             // Extract params and create resource
///             results[0] = Value::Own(ResourceOwn::new(
///                 ctx,
///                 OutputStream(/* data */),
///                 resource_ty.clone(),
///             )?);
///             Ok(())
///         }
///     )
/// )?;
///
/// // Methods:
/// interface.define_func("[method]output_stream.method-name",
///     Func::new(&mut store,
///         FuncType::new([ValueType::Borrow(resource_ty.clone())], []),
///         |ctx, args, _| {
///             let Value::Borrow(res) = &args[0] else {
///                 bail!("Expected Borrow")
///             };
///             let data = res.rep::<OutputStream, _, _>(&ctx.as_context())?;
///             // Use data
///             Ok(())
///         }
///     )
/// )?;
/// ```
///
/// See the waclay examples/resource.rs for a complete working example.
#[derive(Debug)]
pub struct OutputStream {
    // TODO: Add your resource fields
    _placeholder: (),
}

impl OutputStream {
    /// Get the ResourceType for this resource.
    ///
    /// This helper method creates a ResourceType for use in manual
    /// resource registration. See the documentation above for usage.
    pub fn resource_type() -> ResourceType {
        ResourceType::new::<Self>(None)
    }
}

#[derive(Debug, Clone)]
pub enum StreamError {
    LastOperationFailed(Error),
    Closed,
}

impl ComponentType for StreamError {
    fn ty() -> ValueType {
        ValueType::Variant(
            VariantType::new(
                None,
                [
                    VariantCase::new(
                        "last-operation-failed",
                        Some(ValueType::Own(Error::resource_type())),
                    ),
                    VariantCase::new("closed", None),
                ],
            )
            .unwrap(),
        )
    }

    fn from_value(value: &Value) -> Result<Self> {
        if let Value::Variant(variant) = value {
            let discriminant = variant.discriminant();
            let variant_ty = variant.ty();
            let case = &variant_ty.cases()[discriminant];
            let case_name = case.name();
            let payload = variant.value();

            match case_name {
                "last-operation-failed" => {
                    if let Some(payload_value) = payload {
                        let converted = Error::from_value(&payload_value)?;
                        Ok(StreamError::LastOperationFailed(converted))
                    } else {
                        bail!("Expected payload for last-operation-failed case")
                    }
                }
                "closed" => Ok(StreamError::Closed),
                _ => bail!("Unknown variant case: {}", case_name),
            }
        } else {
            bail!("Expected Variant value")
        }
    }

    fn into_value(self) -> Result<Value> {
        let variant_type = VariantType::new(
            None,
            [
                VariantCase::new(
                    "last-operation-failed",
                    Some(ValueType::Own(Error::resource_type())),
                ),
                VariantCase::new("closed", None),
            ],
        )
        .unwrap();

        let (discriminant, payload) = match self {
            StreamError::LastOperationFailed(val) => (0, Some(val.into_value()?)),
            StreamError::Closed => (1, None),
        };

        let variant = Variant::new(variant_type, discriminant, payload)?;
        Ok(Value::Variant(variant))
    }
}

impl UnaryComponentType for StreamError {}

/// Resource type: TerminalInput
///
/// This is a host-managed resource. You should define this type
/// with your actual resource data, then use the manual registration
/// pattern shown below instead of the generated trait.
///
/// Example:
/// ```rust,ignore
/// #[derive(Debug)]
/// pub struct TerminalInput {
///     // Your resource data here
///     value: i32,
/// }
///
/// // Manual registration (replaces generated trait):
/// let resource_ty = ResourceType::new::<TerminalInput>(None);
///
/// // Constructor:
/// interface.define_func("[constructor]terminal_input",
///     Func::new(&mut store,
///         FuncType::new([/* params */], [ValueType::Own(resource_ty.clone())]),
///         move |ctx, args, results| {
///             // Extract params and create resource
///             results[0] = Value::Own(ResourceOwn::new(
///                 ctx,
///                 TerminalInput(/* data */),
///                 resource_ty.clone(),
///             )?);
///             Ok(())
///         }
///     )
/// )?;
///
/// // Methods:
/// interface.define_func("[method]terminal_input.method-name",
///     Func::new(&mut store,
///         FuncType::new([ValueType::Borrow(resource_ty.clone())], []),
///         |ctx, args, _| {
///             let Value::Borrow(res) = &args[0] else {
///                 bail!("Expected Borrow")
///             };
///             let data = res.rep::<TerminalInput, _, _>(&ctx.as_context())?;
///             // Use data
///             Ok(())
///         }
///     )
/// )?;
/// ```
///
/// See the waclay examples/resource.rs for a complete working example.
#[derive(Debug)]
pub struct TerminalInput {
    // TODO: Add your resource fields
    _placeholder: (),
}

impl TerminalInput {
    /// Get the ResourceType for this resource.
    ///
    /// This helper method creates a ResourceType for use in manual
    /// resource registration. See the documentation above for usage.
    pub fn resource_type() -> ResourceType {
        ResourceType::new::<Self>(None)
    }
}

/// Resource type: TerminalOutput
///
/// This is a host-managed resource. You should define this type
/// with your actual resource data, then use the manual registration
/// pattern shown below instead of the generated trait.
///
/// Example:
/// ```rust,ignore
/// #[derive(Debug)]
/// pub struct TerminalOutput {
///     // Your resource data here
///     value: i32,
/// }
///
/// // Manual registration (replaces generated trait):
/// let resource_ty = ResourceType::new::<TerminalOutput>(None);
///
/// // Constructor:
/// interface.define_func("[constructor]terminal_output",
///     Func::new(&mut store,
///         FuncType::new([/* params */], [ValueType::Own(resource_ty.clone())]),
///         move |ctx, args, results| {
///             // Extract params and create resource
///             results[0] = Value::Own(ResourceOwn::new(
///                 ctx,
///                 TerminalOutput(/* data */),
///                 resource_ty.clone(),
///             )?);
///             Ok(())
///         }
///     )
/// )?;
///
/// // Methods:
/// interface.define_func("[method]terminal_output.method-name",
///     Func::new(&mut store,
///         FuncType::new([ValueType::Borrow(resource_ty.clone())], []),
///         |ctx, args, _| {
///             let Value::Borrow(res) = &args[0] else {
///                 bail!("Expected Borrow")
///             };
///             let data = res.rep::<TerminalOutput, _, _>(&ctx.as_context())?;
///             // Use data
///             Ok(())
///         }
///     )
/// )?;
/// ```
///
/// See the waclay examples/resource.rs for a complete working example.
#[derive(Debug)]
pub struct TerminalOutput {
    // TODO: Add your resource fields
    _placeholder: (),
}

impl TerminalOutput {
    /// Get the ResourceType for this resource.
    ///
    /// This helper method creates a ResourceType for use in manual
    /// resource registration. See the documentation above for usage.
    pub fn resource_type() -> ResourceType {
        ResourceType::new::<Self>(None)
    }
}

// ========== Host Imports ==========

/// Host trait for interface: wasi:io/poll@0.2.6
pub trait PollHost {
    fn pollable_block(&mut self, self_: Pollable) -> ();
}

/// Host trait for interface: wasi:io/error@0.2.6
pub trait ErrorHost {}

/// Host trait for interface: wasi:io/streams@0.2.6
pub trait StreamsHost {
    fn input_stream_blocking_read(
        &mut self,
        self_: InputStream,
        len: u64,
    ) -> Result<Vec<u8>, StreamError>;
    fn input_stream_subscribe(&mut self, self_: InputStream) -> Pollable;
    fn output_stream_check_write(&mut self, self_: OutputStream) -> Result<u64, StreamError>;
    fn output_stream_write(
        &mut self,
        self_: OutputStream,
        contents: Vec<u8>,
    ) -> Result<(), StreamError>;
    fn output_stream_blocking_flush(&mut self, self_: OutputStream) -> Result<(), StreamError>;
    fn output_stream_subscribe(&mut self, self_: OutputStream) -> Pollable;
}

/// Host trait for interface: wasi:cli/environment@0.2.6
pub trait EnvironmentHost {
    fn get_environment(&mut self) -> Vec<(String, String)>;
}

/// Host trait for interface: wasi:cli/exit@0.2.6
pub trait ExitHost {
    fn exit(&mut self, status: Result<(), ()>) -> ();
}

/// Host trait for interface: wasi:cli/stdin@0.2.6
pub trait StdinHost {
    fn get_stdin(&mut self) -> InputStream;
}

/// Host trait for interface: wasi:cli/stdout@0.2.6
pub trait StdoutHost {
    fn get_stdout(&mut self) -> OutputStream;
}

/// Host trait for interface: wasi:cli/stderr@0.2.6
pub trait StderrHost {
    fn get_stderr(&mut self) -> OutputStream;
}

/// Host trait for interface: wasi:cli/terminal-input@0.2.6
pub trait TerminalInputHost {}

/// Host trait for interface: wasi:cli/terminal-output@0.2.6
pub trait TerminalOutputHost {}

/// Host trait for interface: wasi:cli/terminal-stdin@0.2.6
pub trait TerminalStdinHost {
    fn get_terminal_stdin(&mut self) -> Option<TerminalInput>;
}

/// Host trait for interface: wasi:cli/terminal-stdout@0.2.6
pub trait TerminalStdoutHost {
    fn get_terminal_stdout(&mut self) -> Option<TerminalOutput>;
}

/// Host trait for interface: wasi:cli/terminal-stderr@0.2.6
pub trait TerminalStderrHost {
    fn get_terminal_stderr(&mut self) -> Option<TerminalOutput>;
}

pub mod imports {
    use super::*;

    // NOTE: This interface contains resources which require manual
    // implementation. See the generated resource type documentation
    // for the correct registration pattern.
    //
    // The generated trait below will not compile correctly for resource
    // methods. Use the manual registration pattern shown in the
    // resource type documentation instead.
    pub fn register_poll_host<T: PollHost + 'static, E: backend::WasmEngine>(
        linker: &mut Linker,
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:io/poll@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        // Register resource: pollable
        host_interface
            .define_resource("pollable", Pollable::resource_type())
            .context("Failed to define resource pollable")?;

        host_interface
            .define_func(
                "[method]pollable.block",
                Func::new(
                    &mut *store,
                    FuncType::new([ValueType::Borrow(Pollable::resource_type())], []),
                    |mut ctx, params, _results| {
                        let self_ = Pollable::from_value(&params[0])?;
                        ctx.data_mut().pollable_block(self_);
                        Ok(())
                    },
                ),
            )
            .context("Failed to define [method]pollable.block function")?;

        Ok(())
    }

    // NOTE: This interface contains resources which require manual
    // implementation. See the generated resource type documentation
    // for the correct registration pattern.
    //
    // The generated trait below will not compile correctly for resource
    // methods. Use the manual registration pattern shown in the
    // resource type documentation instead.
    pub fn register_error_host<T: ErrorHost + 'static, E: backend::WasmEngine>(
        linker: &mut Linker,
        _store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:io/error@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        // Register resource: error
        host_interface
            .define_resource("error", Error::resource_type())
            .context("Failed to define resource error")?;

        Ok(())
    }

    // NOTE: This interface contains resources which require manual
    // implementation. See the generated resource type documentation
    // for the correct registration pattern.
    //
    // The generated trait below will not compile correctly for resource
    // methods. Use the manual registration pattern shown in the
    // resource type documentation instead.
    pub fn register_streams_host<T: StreamsHost + 'static, E: backend::WasmEngine>(
        linker: &mut Linker,
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:io/streams@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        // Register resource: input-stream
        host_interface
            .define_resource("input-stream", InputStream::resource_type())
            .context("Failed to define resource input-stream")?;

        // Register resource: output-stream
        host_interface
            .define_resource("output-stream", OutputStream::resource_type())
            .context("Failed to define resource output-stream")?;

        host_interface
            .define_func(
                "[method]input-stream.blocking-read",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [
                            ValueType::Borrow(InputStream::resource_type()),
                            ValueType::U64,
                        ],
                        [ValueType::Result(ResultType::new(
                            Some(ValueType::List(ListType::new(ValueType::U8))),
                            Some(StreamError::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = InputStream::from_value(&params[0])?;
                        let len = if let Value::U64(x) = &params[1] {
                            *x
                        } else {
                            bail!("Expected u64")
                        };
                        let result = ctx.data_mut().input_stream_blocking_read(self_, len);
                        results[0] = result.into_value()?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define [method]input-stream.blocking-read function")?;

        host_interface
            .define_func(
                "[method]input-stream.subscribe",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [ValueType::Borrow(InputStream::resource_type())],
                        [ValueType::Own(Pollable::resource_type())],
                    ),
                    |mut ctx, params, results| {
                        let self_ = InputStream::from_value(&params[0])?;
                        let result = ctx.data_mut().input_stream_subscribe(self_);
                        results[0] = result.into_value()?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define [method]input-stream.subscribe function")?;

        host_interface
            .define_func(
                "[method]output-stream.check-write",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [ValueType::Borrow(OutputStream::resource_type())],
                        [ValueType::Result(ResultType::new(
                            Some(ValueType::U64),
                            Some(StreamError::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = OutputStream::from_value(&params[0])?;
                        let result = ctx.data_mut().output_stream_check_write(self_);
                        results[0] = result.into_value()?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define [method]output-stream.check-write function")?;

        host_interface
            .define_func(
                "[method]output-stream.write",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [
                            ValueType::Borrow(OutputStream::resource_type()),
                            ValueType::List(ListType::new(ValueType::U8)),
                        ],
                        [ValueType::Result(ResultType::new(
                            None,
                            Some(StreamError::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = OutputStream::from_value(&params[0])?;
                        let contents = Vec::<u8>::from_value(&params[1])?;
                        let result = ctx.data_mut().output_stream_write(self_, contents);
                        results[0] = result.into_value()?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define [method]output-stream.write function")?;

        host_interface
            .define_func(
                "[method]output-stream.blocking-flush",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [ValueType::Borrow(OutputStream::resource_type())],
                        [ValueType::Result(ResultType::new(
                            None,
                            Some(StreamError::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = OutputStream::from_value(&params[0])?;
                        let result = ctx.data_mut().output_stream_blocking_flush(self_);
                        results[0] = result.into_value()?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define [method]output-stream.blocking-flush function")?;

        host_interface
            .define_func(
                "[method]output-stream.subscribe",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [ValueType::Borrow(OutputStream::resource_type())],
                        [ValueType::Own(Pollable::resource_type())],
                    ),
                    |mut ctx, params, results| {
                        let self_ = OutputStream::from_value(&params[0])?;
                        let result = ctx.data_mut().output_stream_subscribe(self_);
                        results[0] = result.into_value()?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define [method]output-stream.subscribe function")?;

        Ok(())
    }

    pub fn register_environment_host<T: EnvironmentHost + 'static, E: backend::WasmEngine>(
        linker: &mut Linker,
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:cli/environment@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        host_interface
            .define_func(
                "get-environment",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [],
                        [ValueType::List(ListType::new(ValueType::Tuple(
                            TupleType::new(None, [ValueType::String, ValueType::String]),
                        )))],
                    ),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_environment();
                        results[0] = result.into_value()?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define get-environment function")?;

        Ok(())
    }

    pub fn register_exit_host<T: ExitHost + 'static, E: backend::WasmEngine>(
        linker: &mut Linker,
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:cli/exit@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        host_interface
            .define_func(
                "exit",
                Func::new(
                    &mut *store,
                    FuncType::new([ValueType::Result(ResultType::new(None, None))], []),
                    |mut ctx, params, _results| {
                        let status = Result::<(), ()>::from_value(&params[0])?;
                        ctx.data_mut().exit(status);
                        Ok(())
                    },
                ),
            )
            .context("Failed to define exit function")?;

        Ok(())
    }

    pub fn register_stdin_host<T: StdinHost + 'static, E: backend::WasmEngine>(
        linker: &mut Linker,
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:cli/stdin@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        host_interface
            .define_func(
                "get-stdin",
                Func::new(
                    &mut *store,
                    FuncType::new([], [ValueType::Own(InputStream::resource_type())]),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_stdin();
                        results[0] = result.into_value()?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define get-stdin function")?;

        Ok(())
    }

    pub fn register_stdout_host<T: StdoutHost + 'static, E: backend::WasmEngine>(
        linker: &mut Linker,
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:cli/stdout@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        host_interface
            .define_func(
                "get-stdout",
                Func::new(
                    &mut *store,
                    FuncType::new([], [ValueType::Own(OutputStream::resource_type())]),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_stdout();
                        results[0] = result.into_value()?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define get-stdout function")?;

        Ok(())
    }

    pub fn register_stderr_host<T: StderrHost + 'static, E: backend::WasmEngine>(
        linker: &mut Linker,
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:cli/stderr@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        host_interface
            .define_func(
                "get-stderr",
                Func::new(
                    &mut *store,
                    FuncType::new([], [ValueType::Own(OutputStream::resource_type())]),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_stderr();
                        results[0] = result.into_value()?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define get-stderr function")?;

        Ok(())
    }

    // NOTE: This interface contains resources which require manual
    // implementation. See the generated resource type documentation
    // for the correct registration pattern.
    //
    // The generated trait below will not compile correctly for resource
    // methods. Use the manual registration pattern shown in the
    // resource type documentation instead.
    pub fn register_terminal_input_host<T: TerminalInputHost + 'static, E: backend::WasmEngine>(
        linker: &mut Linker,
        _store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:cli/terminal-input@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        // Register resource: terminal-input
        host_interface
            .define_resource("terminal-input", TerminalInput::resource_type())
            .context("Failed to define resource terminal-input")?;

        Ok(())
    }

    // NOTE: This interface contains resources which require manual
    // implementation. See the generated resource type documentation
    // for the correct registration pattern.
    //
    // The generated trait below will not compile correctly for resource
    // methods. Use the manual registration pattern shown in the
    // resource type documentation instead.
    pub fn register_terminal_output_host<
        T: TerminalOutputHost + 'static,
        E: backend::WasmEngine,
    >(
        linker: &mut Linker,
        _store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:cli/terminal-output@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        // Register resource: terminal-output
        host_interface
            .define_resource("terminal-output", TerminalOutput::resource_type())
            .context("Failed to define resource terminal-output")?;

        Ok(())
    }

    pub fn register_terminal_stdin_host<T: TerminalStdinHost + 'static, E: backend::WasmEngine>(
        linker: &mut Linker,
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:cli/terminal-stdin@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        host_interface
            .define_func(
                "get-terminal-stdin",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [],
                        [ValueType::Option(OptionType::new(ValueType::Own(
                            TerminalInput::resource_type(),
                        )))],
                    ),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_terminal_stdin();
                        results[0] = result.into_value()?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define get-terminal-stdin function")?;

        Ok(())
    }

    pub fn register_terminal_stdout_host<
        T: TerminalStdoutHost + 'static,
        E: backend::WasmEngine,
    >(
        linker: &mut Linker,
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:cli/terminal-stdout@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        host_interface
            .define_func(
                "get-terminal-stdout",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [],
                        [ValueType::Option(OptionType::new(ValueType::Own(
                            TerminalOutput::resource_type(),
                        )))],
                    ),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_terminal_stdout();
                        results[0] = result.into_value()?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define get-terminal-stdout function")?;

        Ok(())
    }

    pub fn register_terminal_stderr_host<
        T: TerminalStderrHost + 'static,
        E: backend::WasmEngine,
    >(
        linker: &mut Linker,
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:cli/terminal-stderr@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        host_interface
            .define_func(
                "get-terminal-stderr",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [],
                        [ValueType::Option(OptionType::new(ValueType::Own(
                            TerminalOutput::resource_type(),
                        )))],
                    ),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_terminal_stderr();
                        results[0] = result.into_value()?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define get-terminal-stderr function")?;

        Ok(())
    }
}
