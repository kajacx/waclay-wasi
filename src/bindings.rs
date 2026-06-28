// AUTO-GENERATED WIT BINDINGS for wasm-component-layer
// DO NOT EDIT - Regenerate using wit-bindgen-wcl

#![allow(dead_code, unused_imports, ambiguous_glob_reexports)]

use anyhow::*;
use waclay::*;
use wasm_runtime_layer::backend;

// Note: If using flags types, add to your Cargo.toml:
// bitflags = "2.0"

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

    fn from_value(value: &Value, #[allow(unused)] ctx: impl AsContext) -> Result<Self> {
        if let Value::Variant(variant) = value {
            let discriminant = variant.discriminant();
            let variant_ty = variant.ty();
            let case = &variant_ty.cases()[discriminant];
            let case_name = case.name();
            let payload = variant.value();

            match case_name {
                "last-operation-failed" => {
                    if let Some(payload_value) = payload {
                        let converted = crate::WasiP2ErrorResource::from_value(&payload_value, ctx.as_context())?;
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

    fn into_value(self, #[allow(unused)] mut ctx: impl AsContextMut) -> Result<Value> {
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
            StreamError::LastOperationFailed(val) => {
                (0, Some(val.into_value(ctx.as_context_mut())?))
            }
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

pub type Instant = u64;

#[derive(Debug, Clone)]
pub struct Datetime {
    pub seconds: u64,
    pub nanoseconds: u32,
}

impl ComponentType for Datetime {
    fn ty() -> ValueType {
        ValueType::Record(
            RecordType::new(
                None,
                [("seconds", ValueType::U64), ("nanoseconds", ValueType::U32)],
            )
            .unwrap(),
        )
    }

    fn from_value(value: &Value, #[allow(unused)] ctx: impl AsContext) -> Result<Self> {
        if let Value::Record(record) = value {
            let seconds = record
                .field("seconds")
                .ok_or_else(|| anyhow!("Missing 'seconds' field"))?;
            let nanoseconds = record
                .field("nanoseconds")
                .ok_or_else(|| anyhow!("Missing 'nanoseconds' field"))?;

            let seconds = if let Value::U64(x) = seconds {
                x
            } else {
                bail!("Expected u64")
            };
            let nanoseconds = if let Value::U32(x) = nanoseconds {
                x
            } else {
                bail!("Expected u32")
            };

            Ok(Datetime {
                seconds,
                nanoseconds,
            })
        } else {
            bail!("Expected Record value")
        }
    }

    fn into_value(self, #[allow(unused)] mut ctx: impl AsContextMut) -> Result<Value> {
        let record = Record::new(
            RecordType::new(
                None,
                [("seconds", ValueType::U64), ("nanoseconds", ValueType::U32)],
            )
            .unwrap(),
            [
                ("seconds", Value::U64(self.seconds)),
                ("nanoseconds", Value::U32(self.nanoseconds)),
            ],
        )?;
        Ok(Value::Record(record))
    }
}

impl UnaryComponentType for Datetime {}

/// Resource type: crate::WasiP2DescriptorResource
///
/// This is a host-managed resource. You should define this type
/// with your actual resource data, then use the manual registration
/// pattern shown below instead of the generated trait.
///
/// Example:
/// ```rust,ignore
/// #[derive(Debug)]
/// pub struct crate::WasiP2DescriptorResource {
///     // Your resource data here
///     value: i32,
/// }
///
/// // Manual registration (replaces generated trait):
/// let resource_ty = ResourceType::new::<crate::WasiP2DescriptorResource>(None);
///
/// // Constructor:
/// interface.define_func("[constructor]descriptor",
///     Func::new(&mut store,
///         FuncType::new([/* params */], [ValueType::Own(resource_ty.clone())]),
///         move |ctx, args, results| {
///             // Extract params and create resource
///             results[0] = Value::Own(ResourceOwn::new(
///                 ctx,
///                 crate::WasiP2DescriptorResource(/* data */),
///                 resource_ty.clone(),
///             )?);
///             Ok(())
///         }
///     )
/// )?;
///
/// // Methods:
/// interface.define_func("[method]descriptor.method-name",
///     Func::new(&mut store,
///         FuncType::new([ValueType::Borrow(resource_ty.clone())], []),
///         |ctx, args, _| {
///             let Value::Borrow(res) = &args[0] else {
///                 bail!("Expected Borrow")
///             };
///             let data = res.rep::<crate::WasiP2DescriptorResource, _, _>(&ctx.as_context())?;
///             // Use data
///             Ok(())
///         }
///     )
/// )?;
/// ```
///
/// See the waclay examples/resource.rs for a complete working example.
#[derive(Debug)]
pub struct DescriptorResource {
    // TODO: Add your resource fields
    _placeholder: (),
}

impl DescriptorResource {
    /// Get the ResourceType for this resource.
    ///
    /// This helper method creates a ResourceType for use in manual
    /// resource registration. See the documentation above for usage.
    pub fn resource_type() -> ResourceType {
        ResourceType::new::<Self>(None)
    }
}

pub type Filesize = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    Access,
    WouldBlock,
    Already,
    BadDescriptor,
    Busy,
    Deadlock,
    Quota,
    Exist,
    FileTooLarge,
    IllegalByteSequence,
    InProgress,
    Interrupted,
    Invalid,
    Io,
    IsDirectory,
    Loop,
    TooManyLinks,
    MessageSize,
    NameTooLong,
    NoDevice,
    NoEntry,
    NoLock,
    InsufficientMemory,
    InsufficientSpace,
    NotDirectory,
    NotEmpty,
    NotRecoverable,
    Unsupported,
    NoTty,
    NoSuchDevice,
    Overflow,
    NotPermitted,
    Pipe,
    ReadOnly,
    InvalidSeek,
    TextFileBusy,
    CrossDevice,
}

impl ComponentType for ErrorCode {
    fn ty() -> ValueType {
        ValueType::Enum(
            EnumType::new(
                None,
                [
                    "access",
                    "would-block",
                    "already",
                    "bad-descriptor",
                    "busy",
                    "deadlock",
                    "quota",
                    "exist",
                    "file-too-large",
                    "illegal-byte-sequence",
                    "in-progress",
                    "interrupted",
                    "invalid",
                    "io",
                    "is-directory",
                    "loop",
                    "too-many-links",
                    "message-size",
                    "name-too-long",
                    "no-device",
                    "no-entry",
                    "no-lock",
                    "insufficient-memory",
                    "insufficient-space",
                    "not-directory",
                    "not-empty",
                    "not-recoverable",
                    "unsupported",
                    "no-tty",
                    "no-such-device",
                    "overflow",
                    "not-permitted",
                    "pipe",
                    "read-only",
                    "invalid-seek",
                    "text-file-busy",
                    "cross-device",
                ],
            )
            .unwrap(),
        )
    }

    fn from_value(value: &Value, #[allow(unused)] ctx: impl AsContext) -> Result<Self> {
        if let Value::Enum(enum_val) = value {
            let discriminant = enum_val.discriminant();
            match discriminant {
                0 => Ok(ErrorCode::Access),
                1 => Ok(ErrorCode::WouldBlock),
                2 => Ok(ErrorCode::Already),
                3 => Ok(ErrorCode::BadDescriptor),
                4 => Ok(ErrorCode::Busy),
                5 => Ok(ErrorCode::Deadlock),
                6 => Ok(ErrorCode::Quota),
                7 => Ok(ErrorCode::Exist),
                8 => Ok(ErrorCode::FileTooLarge),
                9 => Ok(ErrorCode::IllegalByteSequence),
                10 => Ok(ErrorCode::InProgress),
                11 => Ok(ErrorCode::Interrupted),
                12 => Ok(ErrorCode::Invalid),
                13 => Ok(ErrorCode::Io),
                14 => Ok(ErrorCode::IsDirectory),
                15 => Ok(ErrorCode::Loop),
                16 => Ok(ErrorCode::TooManyLinks),
                17 => Ok(ErrorCode::MessageSize),
                18 => Ok(ErrorCode::NameTooLong),
                19 => Ok(ErrorCode::NoDevice),
                20 => Ok(ErrorCode::NoEntry),
                21 => Ok(ErrorCode::NoLock),
                22 => Ok(ErrorCode::InsufficientMemory),
                23 => Ok(ErrorCode::InsufficientSpace),
                24 => Ok(ErrorCode::NotDirectory),
                25 => Ok(ErrorCode::NotEmpty),
                26 => Ok(ErrorCode::NotRecoverable),
                27 => Ok(ErrorCode::Unsupported),
                28 => Ok(ErrorCode::NoTty),
                29 => Ok(ErrorCode::NoSuchDevice),
                30 => Ok(ErrorCode::Overflow),
                31 => Ok(ErrorCode::NotPermitted),
                32 => Ok(ErrorCode::Pipe),
                33 => Ok(ErrorCode::ReadOnly),
                34 => Ok(ErrorCode::InvalidSeek),
                35 => Ok(ErrorCode::TextFileBusy),
                36 => Ok(ErrorCode::CrossDevice),
                _ => bail!("Invalid enum discriminant: {}", discriminant),
            }
        } else {
            bail!("Expected Enum value")
        }
    }

    fn into_value(self, #[allow(unused)] mut ctx: impl AsContextMut) -> Result<Value> {
        let enum_type = EnumType::new(
            None,
            [
                "access",
                "would-block",
                "already",
                "bad-descriptor",
                "busy",
                "deadlock",
                "quota",
                "exist",
                "file-too-large",
                "illegal-byte-sequence",
                "in-progress",
                "interrupted",
                "invalid",
                "io",
                "is-directory",
                "loop",
                "too-many-links",
                "message-size",
                "name-too-long",
                "no-device",
                "no-entry",
                "no-lock",
                "insufficient-memory",
                "insufficient-space",
                "not-directory",
                "not-empty",
                "not-recoverable",
                "unsupported",
                "no-tty",
                "no-such-device",
                "overflow",
                "not-permitted",
                "pipe",
                "read-only",
                "invalid-seek",
                "text-file-busy",
                "cross-device",
            ],
        )
        .unwrap();

        let discriminant = match self {
            ErrorCode::Access => 0,
            ErrorCode::WouldBlock => 1,
            ErrorCode::Already => 2,
            ErrorCode::BadDescriptor => 3,
            ErrorCode::Busy => 4,
            ErrorCode::Deadlock => 5,
            ErrorCode::Quota => 6,
            ErrorCode::Exist => 7,
            ErrorCode::FileTooLarge => 8,
            ErrorCode::IllegalByteSequence => 9,
            ErrorCode::InProgress => 10,
            ErrorCode::Interrupted => 11,
            ErrorCode::Invalid => 12,
            ErrorCode::Io => 13,
            ErrorCode::IsDirectory => 14,
            ErrorCode::Loop => 15,
            ErrorCode::TooManyLinks => 16,
            ErrorCode::MessageSize => 17,
            ErrorCode::NameTooLong => 18,
            ErrorCode::NoDevice => 19,
            ErrorCode::NoEntry => 20,
            ErrorCode::NoLock => 21,
            ErrorCode::InsufficientMemory => 22,
            ErrorCode::InsufficientSpace => 23,
            ErrorCode::NotDirectory => 24,
            ErrorCode::NotEmpty => 25,
            ErrorCode::NotRecoverable => 26,
            ErrorCode::Unsupported => 27,
            ErrorCode::NoTty => 28,
            ErrorCode::NoSuchDevice => 29,
            ErrorCode::Overflow => 30,
            ErrorCode::NotPermitted => 31,
            ErrorCode::Pipe => 32,
            ErrorCode::ReadOnly => 33,
            ErrorCode::InvalidSeek => 34,
            ErrorCode::TextFileBusy => 35,
            ErrorCode::CrossDevice => 36,
        };

        Ok(Value::Enum(Enum::new(enum_type, discriminant)?))
    }
}

impl UnaryComponentType for ErrorCode {}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DescriptorFlags: u32 {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const FILE_INTEGRITY_SYNC = 1 << 2;
        const DATA_INTEGRITY_SYNC = 1 << 3;
        const REQUESTED_WRITE_SYNC = 1 << 4;
        const MUTATE_DIRECTORY = 1 << 5;
    }
}

impl ComponentType for DescriptorFlags {
    fn ty() -> ValueType {
        ValueType::Flags(
            FlagsType::new(
                None,
                [
                    "read",
                    "write",
                    "file-integrity-sync",
                    "data-integrity-sync",
                    "requested-write-sync",
                    "mutate-directory",
                ],
            )
            .unwrap(),
        )
    }

    fn from_value(value: &Value, #[allow(unused)] ctx: impl AsContext) -> Result<Self> {
        if let Value::Flags(flags_val) = value {
            let mut result = DescriptorFlags::empty();
            let ty = flags_val.ty();
            let count = ty.names().len();
            for i in 0..count {
                if flags_val.get_index(i) {
                    result |= DescriptorFlags::from_bits_truncate(1 << i);
                }
            }
            Ok(result)
        } else {
            bail!("Expected Flags value")
        }
    }

    fn into_value(self, #[allow(unused)] mut ctx: impl AsContextMut) -> Result<Value> {
        let flags_type = FlagsType::new(
            None,
            [
                "read",
                "write",
                "file-integrity-sync",
                "data-integrity-sync",
                "requested-write-sync",
                "mutate-directory",
            ],
        )
        .unwrap();

        let mut flags_val = Flags::new(flags_type);
        for i in 0..6 {
            if self.bits() & (1 << i) != 0 {
                flags_val.set_index(i, true);
            }
        }
        Ok(Value::Flags(flags_val))
    }
}

impl UnaryComponentType for DescriptorFlags {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptorType {
    Unknown,
    BlockDevice,
    CharacterDevice,
    Directory,
    Fifo,
    SymbolicLink,
    RegularFile,
    Socket,
}

impl ComponentType for DescriptorType {
    fn ty() -> ValueType {
        ValueType::Enum(
            EnumType::new(
                None,
                [
                    "unknown",
                    "block-device",
                    "character-device",
                    "directory",
                    "fifo",
                    "symbolic-link",
                    "regular-file",
                    "socket",
                ],
            )
            .unwrap(),
        )
    }

    fn from_value(value: &Value, #[allow(unused)] ctx: impl AsContext) -> Result<Self> {
        if let Value::Enum(enum_val) = value {
            let discriminant = enum_val.discriminant();
            match discriminant {
                0 => Ok(DescriptorType::Unknown),
                1 => Ok(DescriptorType::BlockDevice),
                2 => Ok(DescriptorType::CharacterDevice),
                3 => Ok(DescriptorType::Directory),
                4 => Ok(DescriptorType::Fifo),
                5 => Ok(DescriptorType::SymbolicLink),
                6 => Ok(DescriptorType::RegularFile),
                7 => Ok(DescriptorType::Socket),
                _ => bail!("Invalid enum discriminant: {}", discriminant),
            }
        } else {
            bail!("Expected Enum value")
        }
    }

    fn into_value(self, #[allow(unused)] mut ctx: impl AsContextMut) -> Result<Value> {
        let enum_type = EnumType::new(
            None,
            [
                "unknown",
                "block-device",
                "character-device",
                "directory",
                "fifo",
                "symbolic-link",
                "regular-file",
                "socket",
            ],
        )
        .unwrap();

        let discriminant = match self {
            DescriptorType::Unknown => 0,
            DescriptorType::BlockDevice => 1,
            DescriptorType::CharacterDevice => 2,
            DescriptorType::Directory => 3,
            DescriptorType::Fifo => 4,
            DescriptorType::SymbolicLink => 5,
            DescriptorType::RegularFile => 6,
            DescriptorType::Socket => 7,
        };

        Ok(Value::Enum(Enum::new(enum_type, discriminant)?))
    }
}

impl UnaryComponentType for DescriptorType {}

pub type LinkCount = u64;

#[derive(Debug, Clone)]
pub struct DescriptorStat {
    pub r#type: DescriptorType,
    pub link_count: LinkCount,
    pub size: Filesize,
    pub data_access_timestamp: Option<Datetime>,
    pub data_modification_timestamp: Option<Datetime>,
    pub status_change_timestamp: Option<Datetime>,
}

impl ComponentType for DescriptorStat {
    fn ty() -> ValueType {
        ValueType::Record(
            RecordType::new(
                None,
                [
                    ("type", DescriptorType::ty()),
                    ("link-count", LinkCount::ty()),
                    ("size", Filesize::ty()),
                    (
                        "data-access-timestamp",
                        ValueType::Option(OptionType::new(Datetime::ty())),
                    ),
                    (
                        "data-modification-timestamp",
                        ValueType::Option(OptionType::new(Datetime::ty())),
                    ),
                    (
                        "status-change-timestamp",
                        ValueType::Option(OptionType::new(Datetime::ty())),
                    ),
                ],
            )
            .unwrap(),
        )
    }

    fn from_value(value: &Value, #[allow(unused)] ctx: impl AsContext) -> Result<Self> {
        if let Value::Record(record) = value {
            let r#type = record
                .field("type")
                .ok_or_else(|| anyhow!("Missing 'type' field"))?;
            let link_count = record
                .field("link-count")
                .ok_or_else(|| anyhow!("Missing 'link-count' field"))?;
            let size = record
                .field("size")
                .ok_or_else(|| anyhow!("Missing 'size' field"))?;
            let data_access_timestamp = record
                .field("data-access-timestamp")
                .ok_or_else(|| anyhow!("Missing 'data-access-timestamp' field"))?;
            let data_modification_timestamp = record
                .field("data-modification-timestamp")
                .ok_or_else(|| anyhow!("Missing 'data-modification-timestamp' field"))?;
            let status_change_timestamp = record
                .field("status-change-timestamp")
                .ok_or_else(|| anyhow!("Missing 'status-change-timestamp' field"))?;

            let r#type = DescriptorType::from_value(&r#type, ctx.as_context())?;
            let link_count = LinkCount::from_value(&link_count, ctx.as_context())?;
            let size = Filesize::from_value(&size, ctx.as_context())?;
            let data_access_timestamp =
                Option::<Datetime>::from_value(&data_access_timestamp, ctx.as_context())?;
            let data_modification_timestamp =
                Option::<Datetime>::from_value(&data_modification_timestamp, ctx.as_context())?;
            let status_change_timestamp =
                Option::<Datetime>::from_value(&status_change_timestamp, ctx.as_context())?;

            Ok(DescriptorStat {
                r#type,
                link_count,
                size,
                data_access_timestamp,
                data_modification_timestamp,
                status_change_timestamp,
            })
        } else {
            bail!("Expected Record value")
        }
    }

    fn into_value(self, #[allow(unused)] mut ctx: impl AsContextMut) -> Result<Value> {
        let record = Record::new(
            RecordType::new(
                None,
                [
                    ("type", DescriptorType::ty()),
                    ("link-count", LinkCount::ty()),
                    ("size", Filesize::ty()),
                    (
                        "data-access-timestamp",
                        ValueType::Option(OptionType::new(Datetime::ty())),
                    ),
                    (
                        "data-modification-timestamp",
                        ValueType::Option(OptionType::new(Datetime::ty())),
                    ),
                    (
                        "status-change-timestamp",
                        ValueType::Option(OptionType::new(Datetime::ty())),
                    ),
                ],
            )
            .unwrap(),
            [
                ("type", self.r#type.into_value(ctx.as_context_mut())?),
                (
                    "link-count",
                    self.link_count.into_value(ctx.as_context_mut())?,
                ),
                ("size", self.size.into_value(ctx.as_context_mut())?),
                (
                    "data-access-timestamp",
                    self.data_access_timestamp
                        .into_value(ctx.as_context_mut())?,
                ),
                (
                    "data-modification-timestamp",
                    self.data_modification_timestamp
                        .into_value(ctx.as_context_mut())?,
                ),
                (
                    "status-change-timestamp",
                    self.status_change_timestamp
                        .into_value(ctx.as_context_mut())?,
                ),
            ],
        )?;
        Ok(Value::Record(record))
    }
}

impl UnaryComponentType for DescriptorStat {}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PathFlags: u32 {
        const SYMLINK_FOLLOW = 1 << 0;
    }
}

impl ComponentType for PathFlags {
    fn ty() -> ValueType {
        ValueType::Flags(FlagsType::new(None, ["symlink-follow"]).unwrap())
    }

    fn from_value(value: &Value, #[allow(unused)] ctx: impl AsContext) -> Result<Self> {
        if let Value::Flags(flags_val) = value {
            let mut result = PathFlags::empty();
            let ty = flags_val.ty();
            let count = ty.names().len();
            for i in 0..count {
                if flags_val.get_index(i) {
                    result |= PathFlags::from_bits_truncate(1 << i);
                }
            }
            Ok(result)
        } else {
            bail!("Expected Flags value")
        }
    }

    fn into_value(self, #[allow(unused)] mut ctx: impl AsContextMut) -> Result<Value> {
        let flags_type = FlagsType::new(None, ["symlink-follow"]).unwrap();

        let mut flags_val = Flags::new(flags_type);
        for i in 0..1 {
            if self.bits() & (1 << i) != 0 {
                flags_val.set_index(i, true);
            }
        }
        Ok(Value::Flags(flags_val))
    }
}

impl UnaryComponentType for PathFlags {}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OpenFlags: u32 {
        const CREATE = 1 << 0;
        const DIRECTORY = 1 << 1;
        const EXCLUSIVE = 1 << 2;
        const TRUNCATE = 1 << 3;
    }
}

impl ComponentType for OpenFlags {
    fn ty() -> ValueType {
        ValueType::Flags(
            FlagsType::new(None, ["create", "directory", "exclusive", "truncate"]).unwrap(),
        )
    }

    fn from_value(value: &Value, #[allow(unused)] ctx: impl AsContext) -> Result<Self> {
        if let Value::Flags(flags_val) = value {
            let mut result = OpenFlags::empty();
            let ty = flags_val.ty();
            let count = ty.names().len();
            for i in 0..count {
                if flags_val.get_index(i) {
                    result |= OpenFlags::from_bits_truncate(1 << i);
                }
            }
            Ok(result)
        } else {
            bail!("Expected Flags value")
        }
    }

    fn into_value(self, #[allow(unused)] mut ctx: impl AsContextMut) -> Result<Value> {
        let flags_type =
            FlagsType::new(None, ["create", "directory", "exclusive", "truncate"]).unwrap();

        let mut flags_val = Flags::new(flags_type);
        for i in 0..4 {
            if self.bits() & (1 << i) != 0 {
                flags_val.set_index(i, true);
            }
        }
        Ok(Value::Flags(flags_val))
    }
}

impl UnaryComponentType for OpenFlags {}

#[derive(Debug, Clone)]
pub struct MetadataHashValue {
    pub lower: u64,
    pub upper: u64,
}

impl ComponentType for MetadataHashValue {
    fn ty() -> ValueType {
        ValueType::Record(
            RecordType::new(None, [("lower", ValueType::U64), ("upper", ValueType::U64)]).unwrap(),
        )
    }

    fn from_value(value: &Value, #[allow(unused)] ctx: impl AsContext) -> Result<Self> {
        if let Value::Record(record) = value {
            let lower = record
                .field("lower")
                .ok_or_else(|| anyhow!("Missing 'lower' field"))?;
            let upper = record
                .field("upper")
                .ok_or_else(|| anyhow!("Missing 'upper' field"))?;

            let lower = if let Value::U64(x) = lower {
                x
            } else {
                bail!("Expected u64")
            };
            let upper = if let Value::U64(x) = upper {
                x
            } else {
                bail!("Expected u64")
            };

            Ok(MetadataHashValue { lower, upper })
        } else {
            bail!("Expected Record value")
        }
    }

    fn into_value(self, #[allow(unused)] mut ctx: impl AsContextMut) -> Result<Value> {
        let record = Record::new(
            RecordType::new(None, [("lower", ValueType::U64), ("upper", ValueType::U64)]).unwrap(),
            [
                ("lower", Value::U64(self.lower)),
                ("upper", Value::U64(self.upper)),
            ],
        )?;
        Ok(Value::Record(record))
    }
}

impl UnaryComponentType for MetadataHashValue {}

// ========== Host Imports ==========

/// Host trait for interface: wasi:io/poll@0.2.6
pub trait PollHost {
    fn pollable_block(&mut self, self_: crate::WasiP2PollableResource) -> anyhow::Result<()>;
}

/// Host trait for interface: wasi:io/error@0.2.6
pub trait ErrorHost {}

/// Host trait for interface: wasi:io/streams@0.2.6
pub trait StreamsHost {
    fn input_stream_blocking_read(
        &mut self,
        self_: crate::WasiP2InputStreamResource,
        len: u64,
    ) -> anyhow::Result<Result<Vec<u8>, StreamError>>;
    fn input_stream_subscribe(&mut self, self_: crate::WasiP2InputStreamResource) -> anyhow::Result<crate::WasiP2PollableResource>;
    fn output_stream_check_write(
        &mut self,
        self_: crate::WasiP2OutputStreamResource,
    ) -> anyhow::Result<Result<u64, StreamError>>;
    fn output_stream_write(
        &mut self,
        self_: crate::WasiP2OutputStreamResource,
        contents: Vec<u8>,
    ) -> anyhow::Result<Result<(), StreamError>>;
    fn output_stream_blocking_flush(
        &mut self,
        self_: crate::WasiP2OutputStreamResource,
    ) -> anyhow::Result<Result<(), StreamError>>;
    fn output_stream_subscribe(&mut self, self_: crate::WasiP2OutputStreamResource) -> anyhow::Result<crate::WasiP2PollableResource>;
}

/// Host trait for interface: wasi:cli/environment@0.2.6
pub trait EnvironmentHost {
    fn get_environment(&mut self) -> anyhow::Result<Vec<(String, String)>>;
    fn get_arguments(&mut self) -> anyhow::Result<Vec<String>>;
}

/// Host trait for interface: wasi:cli/exit@0.2.6
pub trait ExitHost {
    fn exit(&mut self, status: Result<(), ()>) -> anyhow::Result<()>;
}

/// Host trait for interface: wasi:cli/stdin@0.2.6
pub trait StdinHost {
    fn get_stdin(&mut self) -> anyhow::Result<crate::WasiP2InputStreamResource>;
}

/// Host trait for interface: wasi:cli/stdout@0.2.6
pub trait StdoutHost {
    fn get_stdout(&mut self) -> anyhow::Result<crate::WasiP2OutputStreamResource>;
}

/// Host trait for interface: wasi:cli/stderr@0.2.6
pub trait StderrHost {
    fn get_stderr(&mut self) -> anyhow::Result<crate::WasiP2OutputStreamResource>;
}

/// Host trait for interface: wasi:cli/terminal-input@0.2.6
pub trait TerminalInputHost {}

/// Host trait for interface: wasi:cli/terminal-output@0.2.6
pub trait TerminalOutputHost {}

/// Host trait for interface: wasi:cli/terminal-stdin@0.2.6
pub trait TerminalStdinHost {
    fn get_terminal_stdin(&mut self) -> anyhow::Result<Option<crate::WasiP2TerminalInputResource>>;
}

/// Host trait for interface: wasi:cli/terminal-stdout@0.2.6
pub trait TerminalStdoutHost {
    fn get_terminal_stdout(&mut self) -> anyhow::Result<Option<crate::WasiP2TerminalOutputResource>>;
}

/// Host trait for interface: wasi:cli/terminal-stderr@0.2.6
pub trait TerminalStderrHost {
    fn get_terminal_stderr(&mut self) -> anyhow::Result<Option<crate::WasiP2TerminalOutputResource>>;
}

/// Host trait for interface: wasi:random/random@0.2.6
pub trait RandomHost {
    fn get_random_u64(&mut self) -> anyhow::Result<u64>;
}

/// Host trait for interface: wasi:clocks/monotonic-clock@0.2.6
pub trait MonotonicClockHost {
    fn now(&mut self) -> anyhow::Result<Instant>;
}

/// Host trait for interface: wasi:clocks/wall-clock@0.2.6
pub trait WallClockHost {
    fn now(&mut self) -> anyhow::Result<Datetime>;
}

/// Host trait for interface: wasi:filesystem/types@0.2.6
pub trait TypesHost {
    fn descriptor_read_via_stream(
        &mut self,
        self_: crate::WasiP2DescriptorResource,
        offset: Filesize,
    ) -> anyhow::Result<Result<crate::WasiP2InputStreamResource, ErrorCode>>;
    fn descriptor_write_via_stream(
        &mut self,
        self_: crate::WasiP2DescriptorResource,
        offset: Filesize,
    ) -> anyhow::Result<Result<crate::WasiP2OutputStreamResource, ErrorCode>>;
    fn descriptor_append_via_stream(
        &mut self,
        self_: crate::WasiP2DescriptorResource,
    ) -> anyhow::Result<Result<crate::WasiP2OutputStreamResource, ErrorCode>>;
    fn descriptor_get_flags(
        &mut self,
        self_: crate::WasiP2DescriptorResource,
    ) -> anyhow::Result<Result<DescriptorFlags, ErrorCode>>;
    fn descriptor_stat(
        &mut self,
        self_: crate::WasiP2DescriptorResource,
    ) -> anyhow::Result<Result<DescriptorStat, ErrorCode>>;
    fn descriptor_open_at(
        &mut self,
        self_: crate::WasiP2DescriptorResource,
        path_flags: PathFlags,
        path: String,
        open_flags: OpenFlags,
        flags: DescriptorFlags,
    ) -> anyhow::Result<Result<crate::WasiP2DescriptorResource, ErrorCode>>;
    fn descriptor_metadata_hash(
        &mut self,
        self_: crate::WasiP2DescriptorResource,
    ) -> anyhow::Result<Result<MetadataHashValue, ErrorCode>>;
}

/// Host trait for interface: wasi:filesystem/preopens@0.2.6
pub trait PreopensHost {
    fn get_directories(&mut self) -> anyhow::Result<Vec<(crate::WasiP2DescriptorResource, String)>>;
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
                    FuncType::new([ValueType::Borrow(crate::WasiP2PollableResource::resource_type())], []),
                    |mut ctx, params, _results| {
                        let self_ = crate::WasiP2PollableResource::from_value(&params[0], ctx.as_context())?;
                        ctx.data_mut().pollable_block(self_)?;
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
        store: &mut Store<T, E>,
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
            .define_resource("input-stream", crate::WasiP2InputStreamResource::resource_type())
            .context("Failed to define resource input-stream")?;

        // Register resource: output-stream
        host_interface
            .define_resource("output-stream", crate::WasiP2OutputStreamResource::resource_type())
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
                        let self_ = crate::WasiP2InputStreamResource::from_value(&params[0], ctx.as_context())?;
                        let len = if let Value::U64(x) = &params[1] {
                            *x
                        } else {
                            bail!("Expected u64")
                        };
                        let result = ctx.data_mut().input_stream_blocking_read(self_, len)?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
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
                        [ValueType::Borrow(crate::WasiP2InputStreamResource::resource_type())],
                        [ValueType::Own(crate::WasiP2PollableResource::resource_type())],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2InputStreamResource::from_value(&params[0], ctx.as_context())?;
                        let result = ctx.data_mut().input_stream_subscribe(self_)?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
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
                        [ValueType::Borrow(crate::WasiP2OutputStreamResource::resource_type())],
                        [ValueType::Result(ResultType::new(
                            Some(ValueType::U64),
                            Some(StreamError::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2OutputStreamResource::from_value(&params[0], ctx.as_context())?;
                        let result = ctx.data_mut().output_stream_check_write(self_)?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
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
                        let self_ = crate::WasiP2OutputStreamResource::from_value(&params[0], ctx.as_context())?;
                        let contents = Vec::<u8>::from_value(&params[1], ctx.as_context())?;
                        let result = ctx.data_mut().output_stream_write(self_, contents)?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
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
                        [ValueType::Borrow(crate::WasiP2OutputStreamResource::resource_type())],
                        [ValueType::Result(ResultType::new(
                            None,
                            Some(StreamError::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2OutputStreamResource::from_value(&params[0], ctx.as_context())?;
                        let result = ctx.data_mut().output_stream_blocking_flush(self_)?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
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
                        [ValueType::Borrow(crate::WasiP2OutputStreamResource::resource_type())],
                        [ValueType::Own(crate::WasiP2PollableResource::resource_type())],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2OutputStreamResource::from_value(&params[0], ctx.as_context())?;
                        let result = ctx.data_mut().output_stream_subscribe(self_)?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
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
                        let result = ctx.data_mut().get_environment()?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define get-environment function")?;

        host_interface
            .define_func(
                "get-arguments",
                Func::new(
                    &mut *store,
                    FuncType::new([], [ValueType::List(ListType::new(ValueType::String))]),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_arguments()?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define get-arguments function")?;

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
                        let status = Result::<(), ()>::from_value(&params[0], ctx.as_context())?;
                        ctx.data_mut().exit(status)?;
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
                    FuncType::new([], [ValueType::Own(crate::WasiP2InputStreamResource::resource_type())]),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_stdin()?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
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
                    FuncType::new([], [ValueType::Own(crate::WasiP2OutputStreamResource::resource_type())]),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_stdout()?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
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
                    FuncType::new([], [ValueType::Own(crate::WasiP2OutputStreamResource::resource_type())]),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_stderr()?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
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
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:cli/terminal-input@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        // Register resource: terminal-input
        host_interface
            .define_resource("terminal-input", crate::WasiP2TerminalInputResource::resource_type())
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
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:cli/terminal-output@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        // Register resource: terminal-output
        host_interface
            .define_resource("terminal-output", crate::WasiP2TerminalOutputResource::resource_type())
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
                        let result = ctx.data_mut().get_terminal_stdin()?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
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
                        let result = ctx.data_mut().get_terminal_stdout()?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
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
                        let result = ctx.data_mut().get_terminal_stderr()?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define get-terminal-stderr function")?;

        Ok(())
    }

    pub fn register_random_host<T: RandomHost + 'static, E: backend::WasmEngine>(
        linker: &mut Linker,
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:random/random@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        host_interface
            .define_func(
                "get-random-u64",
                Func::new(
                    &mut *store,
                    FuncType::new([], [ValueType::U64]),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_random_u64()?;
                        results[0] = Value::U64(result);
                        Ok(())
                    },
                ),
            )
            .context("Failed to define get-random-u64 function")?;

        Ok(())
    }

    pub fn register_monotonic_clock_host<
        T: MonotonicClockHost + 'static,
        E: backend::WasmEngine,
    >(
        linker: &mut Linker,
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:clocks/monotonic-clock@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        host_interface
            .define_func(
                "now",
                Func::new(
                    &mut *store,
                    FuncType::new([], [Instant::ty()]),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().now()?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define now function")?;

        Ok(())
    }

    pub fn register_wall_clock_host<T: WallClockHost + 'static, E: backend::WasmEngine>(
        linker: &mut Linker,
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:clocks/wall-clock@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        host_interface
            .define_func(
                "now",
                Func::new(
                    &mut *store,
                    FuncType::new([], [Datetime::ty()]),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().now()?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define now function")?;

        Ok(())
    }

    // NOTE: This interface contains resources which require manual
    // implementation. See the generated resource type documentation
    // for the correct registration pattern.
    //
    // The generated trait below will not compile correctly for resource
    // methods. Use the manual registration pattern shown in the
    // resource type documentation instead.
    pub fn register_types_host<T: TypesHost + 'static, E: backend::WasmEngine>(
        linker: &mut Linker,
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:filesystem/types@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        // Register resource: descriptor
        host_interface
            .define_resource("descriptor", crate::WasiP2DescriptorResource::resource_type())
            .context("Failed to define resource descriptor")?;

        host_interface
            .define_func(
                "[method]descriptor.read-via-stream",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [
                            ValueType::Borrow(crate::WasiP2DescriptorResource::resource_type()),
                            Filesize::ty(),
                        ],
                        [ValueType::Result(ResultType::new(
                            Some(ValueType::Own(crate::WasiP2InputStreamResource::resource_type())),
                            Some(ErrorCode::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2DescriptorResource::from_value(&params[0], ctx.as_context())?;
                        let offset = Filesize::from_value(&params[1], ctx.as_context())?;
                        let result = ctx.data_mut().descriptor_read_via_stream(self_, offset)?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define [method]descriptor.read-via-stream function")?;

        host_interface
            .define_func(
                "[method]descriptor.write-via-stream",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [
                            ValueType::Borrow(crate::WasiP2DescriptorResource::resource_type()),
                            Filesize::ty(),
                        ],
                        [ValueType::Result(ResultType::new(
                            Some(ValueType::Own(crate::WasiP2OutputStreamResource::resource_type())),
                            Some(ErrorCode::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2DescriptorResource::from_value(&params[0], ctx.as_context())?;
                        let offset = Filesize::from_value(&params[1], ctx.as_context())?;
                        let result = ctx.data_mut().descriptor_write_via_stream(self_, offset)?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define [method]descriptor.write-via-stream function")?;

        host_interface
            .define_func(
                "[method]descriptor.append-via-stream",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [ValueType::Borrow(crate::WasiP2DescriptorResource::resource_type())],
                        [ValueType::Result(ResultType::new(
                            Some(ValueType::Own(crate::WasiP2OutputStreamResource::resource_type())),
                            Some(ErrorCode::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2DescriptorResource::from_value(&params[0], ctx.as_context())?;
                        let result = ctx.data_mut().descriptor_append_via_stream(self_)?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define [method]descriptor.append-via-stream function")?;

        host_interface
            .define_func(
                "[method]descriptor.get-flags",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [ValueType::Borrow(crate::WasiP2DescriptorResource::resource_type())],
                        [ValueType::Result(ResultType::new(
                            Some(DescriptorFlags::ty()),
                            Some(ErrorCode::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2DescriptorResource::from_value(&params[0], ctx.as_context())?;
                        let result = ctx.data_mut().descriptor_get_flags(self_)?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define [method]descriptor.get-flags function")?;

        host_interface
            .define_func(
                "[method]descriptor.stat",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [ValueType::Borrow(crate::WasiP2DescriptorResource::resource_type())],
                        [ValueType::Result(ResultType::new(
                            Some(DescriptorStat::ty()),
                            Some(ErrorCode::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2DescriptorResource::from_value(&params[0], ctx.as_context())?;
                        let result = ctx.data_mut().descriptor_stat(self_)?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define [method]descriptor.stat function")?;

        host_interface
            .define_func(
                "[method]descriptor.open-at",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [
                            ValueType::Borrow(crate::WasiP2DescriptorResource::resource_type()),
                            PathFlags::ty(),
                            ValueType::String,
                            OpenFlags::ty(),
                            DescriptorFlags::ty(),
                        ],
                        [ValueType::Result(ResultType::new(
                            Some(ValueType::Own(crate::WasiP2DescriptorResource::resource_type())),
                            Some(ErrorCode::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2DescriptorResource::from_value(&params[0], ctx.as_context())?;
                        let path_flags = PathFlags::from_value(&params[1], ctx.as_context())?;
                        let path = if let Value::String(s) = &params[2] {
                            s.to_string()
                        } else {
                            bail!("Expected string")
                        };
                        let open_flags = OpenFlags::from_value(&params[3], ctx.as_context())?;
                        let flags = DescriptorFlags::from_value(&params[4], ctx.as_context())?;
                        let result = ctx
                            .data_mut()
                            .descriptor_open_at(self_, path_flags, path, open_flags, flags)?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define [method]descriptor.open-at function")?;

        host_interface
            .define_func(
                "[method]descriptor.metadata-hash",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [ValueType::Borrow(crate::WasiP2DescriptorResource::resource_type())],
                        [ValueType::Result(ResultType::new(
                            Some(MetadataHashValue::ty()),
                            Some(ErrorCode::ty()),
                        ))],
                    ),
                    |mut ctx, params, results| {
                        let self_ = crate::WasiP2DescriptorResource::from_value(&params[0], ctx.as_context())?;
                        let result = ctx.data_mut().descriptor_metadata_hash(self_)?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define [method]descriptor.metadata-hash function")?;

        Ok(())
    }

    pub fn register_preopens_host<T: PreopensHost + 'static, E: backend::WasmEngine>(
        linker: &mut Linker,
        store: &mut Store<T, E>,
    ) -> Result<()> {
        let host_interface = linker
            .define_instance("wasi:filesystem/preopens@0.2.6".try_into().unwrap())
            .context("Failed to define host interface")?;

        host_interface
            .define_func(
                "get-directories",
                Func::new(
                    &mut *store,
                    FuncType::new(
                        [],
                        [ValueType::List(ListType::new(ValueType::Tuple(
                            TupleType::new(
                                None,
                                [
                                    ValueType::Own(crate::WasiP2DescriptorResource::resource_type()),
                                    ValueType::String,
                                ],
                            ),
                        )))],
                    ),
                    |mut ctx, params, results| {
                        let result = ctx.data_mut().get_directories()?;
                        results[0] = result.into_value(ctx.as_context_mut())?;
                        Ok(())
                    },
                ),
            )
            .context("Failed to define get-directories function")?;

        Ok(())
    }
}
