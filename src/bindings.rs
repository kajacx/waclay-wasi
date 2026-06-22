// AUTO-GENERATED WIT BINDINGS for wasm-component-layer
// DO NOT EDIT - Regenerate using wit-bindgen-wcl

#![allow(dead_code, unused_imports, ambiguous_glob_reexports)]

use crate::ResourceConvert;
use waclay::anyhow::*;
use waclay::*;
use wasm_runtime_layer::backend;

// ========== Type Definitions ==========

/// Resource type: crate::WasiP2PollableResource
///
/// This is a host-managed resource. You should define this type
/// with your actual resource data, then use the manual registration
/// pattern shown below instead of the generated trait.
///
/// Example:
/// ```rust,ignore
/// #[derive(Debug)]
/// pub struct crate::WasiP2PollableResource {
///     // Your resource data here
///     value: i32,
/// }
///
/// // Manual registration (replaces generated trait):
/// let resource_ty = ResourceType::new::<crate::WasiP2PollableResource>(None);
///
/// // Constructor:
/// interface.define_func("[constructor]pollable",
///     Func::new(&mut store,
///         FuncType::new([/* params */], [ValueType::Own(resource_ty.clone())]),
///         move |ctx, args, results| {
///             // Extract params and create resource
///             results[0] = Value::Own(ResourceOwn::new(
///                 ctx,
///                 crate::WasiP2PollableResource(/* data */),
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
///             let data = res.rep::<crate::WasiP2PollableResource, _, _>(&ctx.as_context())?;
///             // Use data
///             Ok(())
///         }
///     )
/// )?;
/// ```
///
/// See the waclay examples/resource.rs for a complete working example.
#[derive(Debug)]
pub struct PollableResource {
    // TODO: Add your resource fields
    _placeholder: (),
}

impl PollableResource {
    /// Get the ResourceType for this resource.
    ///
    /// This helper method creates a ResourceType for use in manual
    /// resource registration. See the documentation above for usage.
    pub fn resource_type() -> ResourceType {
        ResourceType::new::<Self>(None)
    }
}

/// Resource type: crate::WasiP2ErrorResource
///
/// This is a host-managed resource. You should define this type
/// with your actual resource data, then use the manual registration
/// pattern shown below instead of the generated trait.
///
/// Example:
/// ```rust,ignore
/// #[derive(Debug)]
/// pub struct crate::WasiP2ErrorResource {
///     // Your resource data here
///     value: i32,
/// }
///
/// // Manual registration (replaces generated trait):
/// let resource_ty = ResourceType::new::<crate::WasiP2ErrorResource>(None);
///
/// // Constructor:
/// interface.define_func("[constructor]error",
///     Func::new(&mut store,
///         FuncType::new([/* params */], [ValueType::Own(resource_ty.clone())]),
///         move |ctx, args, results| {
///             // Extract params and create resource
///             results[0] = Value::Own(ResourceOwn::new(
///                 ctx,
///                 crate::WasiP2ErrorResource(/* data */),
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
///             let data = res.rep::<crate::WasiP2ErrorResource, _, _>(&ctx.as_context())?;
///             // Use data
///             Ok(())
///         }
///     )
/// )?;
/// ```
///
/// See the waclay examples/resource.rs for a complete working example.
#[derive(Debug)]
pub struct ErrorResource {
    // TODO: Add your resource fields
    _placeholder: (),
}

impl ErrorResource {
    /// Get the ResourceType for this resource.
    ///
    /// This helper method creates a ResourceType for use in manual
    /// resource registration. See the documentation above for usage.
    pub fn resource_type() -> ResourceType {
        ResourceType::new::<Self>(None)
    }
}

/// Resource type: crate::WasiP2InputStreamResource
///
/// This is a host-managed resource. You should define this type
/// with your actual resource data, then use the manual registration
/// pattern shown below instead of the generated trait.
///
/// Example:
/// ```rust,ignore
/// #[derive(Debug)]
/// pub struct crate::WasiP2InputStreamResource {
///     // Your resource data here
///     value: i32,
/// }
///
/// // Manual registration (replaces generated trait):
/// let resource_ty = ResourceType::new::<crate::WasiP2InputStreamResource>(None);
///
/// // Constructor:
/// interface.define_func("[constructor]input_stream",
///     Func::new(&mut store,
///         FuncType::new([/* params */], [ValueType::Own(resource_ty.clone())]),
///         move |ctx, args, results| {
///             // Extract params and create resource
///             results[0] = Value::Own(ResourceOwn::new(
///                 ctx,
///                 crate::WasiP2InputStreamResource(/* data */),
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
///             let data = res.rep::<crate::WasiP2InputStreamResource, _, _>(&ctx.as_context())?;
///             // Use data
///             Ok(())
///         }
///     )
/// )?;
/// ```
///
/// See the waclay examples/resource.rs for a complete working example.
#[derive(Debug)]
pub struct InputStreamResource {
    // TODO: Add your resource fields
    _placeholder: (),
}

impl InputStreamResource {
    /// Get the ResourceType for this resource.
    ///
    /// This helper method creates a ResourceType for use in manual
    /// resource registration. See the documentation above for usage.
    pub fn resource_type() -> ResourceType {
        ResourceType::new::<Self>(None)
    }
}

/// Resource type: crate::WasiP2OutputStreamResource
///
/// This is a host-managed resource. You should define this type
/// with your actual resource data, then use the manual registration
/// pattern shown below instead of the generated trait.
///
/// Example:
/// ```rust,ignore
/// #[derive(Debug)]
/// pub struct crate::WasiP2OutputStreamResource {
///     // Your resource data here
///     value: i32,
/// }
///
/// // Manual registration (replaces generated trait):
/// let resource_ty = ResourceType::new::<crate::WasiP2OutputStreamResource>(None);
///
/// // Constructor:
/// interface.define_func("[constructor]output_stream",
///     Func::new(&mut store,
///         FuncType::new([/* params */], [ValueType::Own(resource_ty.clone())]),
///         move |ctx, args, results| {
///             // Extract params and create resource
///             results[0] = Value::Own(ResourceOwn::new(
///                 ctx,
///                 crate::WasiP2OutputStreamResource(/* data */),
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
///             let data = res.rep::<crate::WasiP2OutputStreamResource, _, _>(&ctx.as_context())?;
///             // Use data
///             Ok(())
///         }
///     )
/// )?;
/// ```
///
/// See the waclay examples/resource.rs for a complete working example.
#[derive(Debug)]
pub struct OutputStreamResource {
    // TODO: Add your resource fields
    _placeholder: (),
}

impl OutputStreamResource {
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
    LastOperationFailed(crate::WasiP2ErrorResource),
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
                        Some(ValueType::Own(crate::WasiP2ErrorResource::resource_type())),
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
                        let converted = crate::WasiP2ErrorResource::from_value(&payload_value)?;
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
                    Some(ValueType::Own(crate::WasiP2ErrorResource::resource_type())),
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

/// Resource type: crate::WasiP2TerminalInputResource
///
/// This is a host-managed resource. You should define this type
/// with your actual resource data, then use the manual registration
/// pattern shown below instead of the generated trait.
///
/// Example:
/// ```rust,ignore
/// #[derive(Debug)]
/// pub struct crate::WasiP2TerminalInputResource {
///     // Your resource data here
///     value: i32,
/// }
///
/// // Manual registration (replaces generated trait):
/// let resource_ty = ResourceType::new::<crate::WasiP2TerminalInputResource>(None);
///
/// // Constructor:
/// interface.define_func("[constructor]terminal_input",
///     Func::new(&mut store,
///         FuncType::new([/* params */], [ValueType::Own(resource_ty.clone())]),
///         move |ctx, args, results| {
///             // Extract params and create resource
///             results[0] = Value::Own(ResourceOwn::new(
///                 ctx,
///                 crate::WasiP2TerminalInputResource(/* data */),
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
///             let data = res.rep::<crate::WasiP2TerminalInputResource, _, _>(&ctx.as_context())?;
///             // Use data
///             Ok(())
///         }
///     )
/// )?;
/// ```
///
/// See the waclay examples/resource.rs for a complete working example.
#[derive(Debug)]
pub struct TerminalInputResource {
    // TODO: Add your resource fields
    _placeholder: (),
}

impl TerminalInputResource {
    /// Get the ResourceType for this resource.
    ///
    /// This helper method creates a ResourceType for use in manual
    /// resource registration. See the documentation above for usage.
    pub fn resource_type() -> ResourceType {
        ResourceType::new::<Self>(None)
    }
}

/// Resource type: crate::WasiP2TerminalOutputResource
///
/// This is a host-managed resource. You should define this type
/// with your actual resource data, then use the manual registration
/// pattern shown below instead of the generated trait.
///
/// Example:
/// ```rust,ignore
/// #[derive(Debug)]
/// pub struct crate::WasiP2TerminalOutputResource {
///     // Your resource data here
///     value: i32,
/// }
///
/// // Manual registration (replaces generated trait):
/// let resource_ty = ResourceType::new::<crate::WasiP2TerminalOutputResource>(None);
///
/// // Constructor:
/// interface.define_func("[constructor]terminal_output",
///     Func::new(&mut store,
///         FuncType::new([/* params */], [ValueType::Own(resource_ty.clone())]),
///         move |ctx, args, results| {
///             // Extract params and create resource
///             results[0] = Value::Own(ResourceOwn::new(
///                 ctx,
///                 crate::WasiP2TerminalOutputResource(/* data */),
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
///             let data = res.rep::<crate::WasiP2TerminalOutputResource, _, _>(&ctx.as_context())?;
///             // Use data
///             Ok(())
///         }
///     )
/// )?;
/// ```
///
/// See the waclay examples/resource.rs for a complete working example.
#[derive(Debug)]
pub struct TerminalOutputResource {
    // TODO: Add your resource fields
    _placeholder: (),
}

impl TerminalOutputResource {
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
    fn pollable_block(&mut self, self_: crate::WasiP2PollableResource) -> ();
}

/// Host trait for interface: wasi:io/error@0.2.6
pub trait ErrorHost {}

/// Host trait for interface: wasi:io/streams@0.2.6
pub trait StreamsHost {
    fn input_stream_blocking_read(
        &mut self,
        self_: crate::WasiP2InputStreamResource,
        len: u64,
    ) -> Result<Vec<u8>, StreamError>;
    fn input_stream_subscribe(
        &mut self,
        self_: crate::WasiP2InputStreamResource,
    ) -> crate::WasiP2PollableResource;
    fn output_stream_check_write(
        &mut self,
        self_: crate::WasiP2OutputStreamResource,
    ) -> Result<u64, StreamError>;
    fn output_stream_write(
        &mut self,
        self_: crate::WasiP2OutputStreamResource,
        contents: Vec<u8>,
    ) -> Result<(), StreamError>;
    fn output_stream_blocking_flush(
        &mut self,
        self_: crate::WasiP2OutputStreamResource,
    ) -> Result<(), StreamError>;
    fn output_stream_subscribe(
        &mut self,
        self_: crate::WasiP2OutputStreamResource,
    ) -> crate::WasiP2PollableResource;
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
    fn get_stdin(&mut self) -> crate::WasiP2InputStreamResource;
}

/// Host trait for interface: wasi:cli/stdout@0.2.6
pub trait StdoutHost {
    fn get_stdout(&mut self) -> crate::WasiP2OutputStreamResource;
}

/// Host trait for interface: wasi:cli/stderr@0.2.6
pub trait StderrHost {
    fn get_stderr(&mut self) -> crate::WasiP2OutputStreamResource;
}

/// Host trait for interface: wasi:cli/terminal-input@0.2.6
pub trait TerminalInputHost {}

/// Host trait for interface: wasi:cli/terminal-output@0.2.6
pub trait TerminalOutputHost {}

/// Host trait for interface: wasi:cli/terminal-stdin@0.2.6
pub trait TerminalStdinHost {
    fn get_terminal_stdin(&mut self) -> Option<crate::WasiP2TerminalInputResource>;
}

/// Host trait for interface: wasi:cli/terminal-stdout@0.2.6
pub trait TerminalStdoutHost {
    fn get_terminal_stdout(&mut self) -> Option<crate::WasiP2TerminalOutputResource>;
}

/// Host trait for interface: wasi:cli/terminal-stderr@0.2.6
pub trait TerminalStderrHost {
    fn get_terminal_stderr(&mut self) -> Option<crate::WasiP2TerminalOutputResource>;
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
            .define_resource("pollable", crate::WasiP2PollableResource::resource_type())
            .context("Failed to define resource pollable")?;

        host_interface
            .define_func(
                "[method]pollable.block",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [ValueType::Borrow(
                            crate::WasiP2PollableResource::resource_type(),
                        )],
                        [],
                    ),
                    |mut ctx, params, _results| {
                        let self_ = crate::WasiP2PollableResource::from_value(&params[0])?;
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
            .define_resource("error", crate::WasiP2ErrorResource::resource_type())
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
            .define_resource(
                "input-stream",
                crate::WasiP2InputStreamResource::resource_type(),
            )
            .context("Failed to define resource input-stream")?;

        // Register resource: output-stream
        host_interface
            .define_resource(
                "output-stream",
                crate::WasiP2OutputStreamResource::resource_type(),
            )
            .context("Failed to define resource output-stream")?;

        host_interface
            .define_func(
                "[method]input-stream.blocking-read",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [
                            ValueType::Borrow(crate::WasiP2InputStreamResource::resource_type()),
                            ValueType::U64,
                        ],
                        [ValueType::Result(ResultType::new(
                            Some(ValueType::List(ListType::new(ValueType::U8))),
                            Some(StreamError::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2InputStreamResource::from_value(&params[0])?;
                        let len = if let Value::U64(x) = &params[1] {
                            *x
                        } else {
                            bail!("Expected u64")
                        };
                        let result = ctx.data_mut().input_stream_blocking_read(self_, len);
                        results[0] = result.to_value(ctx)?;
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
                        [ValueType::Borrow(
                            crate::WasiP2InputStreamResource::resource_type(),
                        )],
                        [ValueType::Own(
                            crate::WasiP2PollableResource::resource_type(),
                        )],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2InputStreamResource::from_value(&params[0])?;
                        let result = ctx.data_mut().input_stream_subscribe(self_);
                        results[0] = result.to_value(ctx)?;
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
                        [ValueType::Borrow(
                            crate::WasiP2OutputStreamResource::resource_type(),
                        )],
                        [ValueType::Result(ResultType::new(
                            Some(ValueType::U64),
                            Some(StreamError::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2OutputStreamResource::from_value(&params[0])?;
                        let result = ctx.data_mut().output_stream_check_write(self_);
                        results[0] = result.to_value(ctx)?;
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
                            ValueType::Borrow(crate::WasiP2OutputStreamResource::resource_type()),
                            ValueType::List(ListType::new(ValueType::U8)),
                        ],
                        [ValueType::Result(ResultType::new(
                            None,
                            Some(StreamError::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2OutputStreamResource::from_value(&params[0])?;
                        let contents = Vec::<u8>::from_value(&params[1])?;
                        let result = ctx.data_mut().output_stream_write(self_, contents);
                        results[0] = result.to_value(ctx)?;
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
                        [ValueType::Borrow(
                            crate::WasiP2OutputStreamResource::resource_type(),
                        )],
                        [ValueType::Result(ResultType::new(
                            None,
                            Some(StreamError::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2OutputStreamResource::from_value(&params[0])?;
                        let result = ctx.data_mut().output_stream_blocking_flush(self_);
                        results[0] = result.to_value(ctx)?;
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
                        [ValueType::Borrow(
                            crate::WasiP2OutputStreamResource::resource_type(),
                        )],
                        [ValueType::Own(
                            crate::WasiP2PollableResource::resource_type(),
                        )],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2OutputStreamResource::from_value(&params[0])?;
                        let result = ctx.data_mut().output_stream_subscribe(self_);
                        results[0] = result.to_value(ctx)?;
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
                        results[0] = result.to_value(ctx)?;
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
                    FuncType::new(
                        [],
                        [ValueType::Own(
                            crate::WasiP2InputStreamResource::resource_type(),
                        )],
                    ),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_stdin();
                        results[0] = result.to_value(ctx)?;
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
                    FuncType::new(
                        [],
                        [ValueType::Own(
                            crate::WasiP2OutputStreamResource::resource_type(),
                        )],
                    ),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_stdout();
                        results[0] = result.to_value(ctx)?;
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
                    FuncType::new(
                        [],
                        [ValueType::Own(
                            crate::WasiP2OutputStreamResource::resource_type(),
                        )],
                    ),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_stderr();
                        results[0] = result.to_value(ctx)?;
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
            .define_resource(
                "terminal-input",
                crate::WasiP2TerminalInputResource::resource_type(),
            )
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
            .define_resource(
                "terminal-output",
                crate::WasiP2TerminalOutputResource::resource_type(),
            )
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
                            crate::WasiP2TerminalInputResource::resource_type(),
                        )))],
                    ),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_terminal_stdin();
                        results[0] = result.to_value(ctx)?;
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
                            crate::WasiP2TerminalOutputResource::resource_type(),
                        )))],
                    ),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_terminal_stdout();
                        results[0] = result.to_value(ctx)?;
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
                            crate::WasiP2TerminalOutputResource::resource_type(),
                        )))],
                    ),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_terminal_stderr();
                        results[0] = result.to_value(ctx)?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define get-terminal-stderr function")?;

        Ok(())
    }
}
