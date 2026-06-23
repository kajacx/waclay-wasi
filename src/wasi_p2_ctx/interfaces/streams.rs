use std::io::{Read, Write};

use crate::*;

pub trait WasiP2OutputStream {
    /// Return how many bytes can be written into this stream
    fn output_stream_check_write(&mut self) -> Result<u64, bindings::StreamError>;

    /// Write bytes into the stream, if the write is not blocking ,also implement `output_stream_blocking_flush`
    fn output_stream_write(&mut self, contents: Vec<u8>) -> Result<(), bindings::StreamError>;

    /// Blocking wait for bytes to be written in case `output_stream_write` implementation was not blocking
    fn output_stream_blocking_flush(&mut self) -> Result<(), bindings::StreamError> {
        Ok(())
    }
}

impl<T: WasiP2CtxHolder> crate::bindings::StreamsHost for T {
    fn input_stream_blocking_read(
        &mut self,
        _self_: WasiP2InputStreamResource,
        len: u64,
    ) -> Result<Vec<u8>, bindings::StreamError> {
        let mut bytes = vec![0u8; len as usize];
        let result = std::io::stdin().read(&mut bytes);
        match result {
            Ok(bytes_read) => {
                bytes.truncate(bytes_read);
                Ok(bytes)
            }
            Err(error) => {
                eprintln!("Read stdio failed: {error:?}");
                Err(bindings::StreamError::Closed)
            }
        }
    }

    fn input_stream_subscribe(
        &mut self,
        _self_: WasiP2InputStreamResource,
    ) -> WasiP2PollableResource {
        WasiP2PollableResource {}
    }

    fn output_stream_check_write(
        &mut self,
        self_: WasiP2OutputStreamResource,
    ) -> Result<u64, bindings::StreamError> {
        match self_ {
            // WasiP2OutputStreamResource::Stdout => self.get_ctx_mut().stdout.
            _ => todo!(),
        }
    }

    fn output_stream_write(
        &mut self,
        self_: WasiP2OutputStreamResource,
        contents: Vec<u8>,
    ) -> Result<(), bindings::StreamError> {
        match self_ {
            // TODO: better error handling
            WasiP2OutputStreamResource::Stdout => {}
            WasiP2OutputStreamResource::Stderr => {
                std::io::stderr().write_all(&contents).map_err(|err| {
                    eprintln!("Unexpected error when writing to stderr: {err:?}");
                    bindings::StreamError::Closed
                })
            }
        }
    }

    fn output_stream_blocking_flush(
        &mut self,
        _self_: WasiP2OutputStreamResource,
    ) -> Result<(), bindings::StreamError> {
        Ok(())
    }

    fn output_stream_subscribe(
        &mut self,
        _self_: WasiP2OutputStreamResource,
    ) -> WasiP2PollableResource {
        WasiP2PollableResource {}
    }
}

pub mod internal {
    use super::*;

    pub struct OutputStreamEmpty {}

    impl WasiP2OutputStream for OutputStreamEmpty {
        fn output_stream_check_write(&mut self) -> Result<u64, bindings::StreamError> {
            Ok(4 * 1024 * 1024)
        }

        fn output_stream_write(&mut self, contents: Vec<u8>) -> Result<(), bindings::StreamError> {
            Ok(())
        }
    }

    pub struct OutputStreamStdout {}

    impl WasiP2OutputStream for OutputStreamStdout {
        fn output_stream_check_write(&mut self) -> Result<u64, bindings::StreamError> {
            Ok(4 * 1024 * 1024)
        }

        fn output_stream_write(&mut self, contents: Vec<u8>) -> Result<(), bindings::StreamError> {
            std::io::stdout().write_all(&contents).map_err(|err| {
                eprintln!("Unexpected error when writing to stdout: {err:?}");
                bindings::StreamError::Closed
            })
        }
    }

    pub struct OutputStreamStderr {}

    impl WasiP2OutputStream for OutputStreamStderr {
        fn output_stream_check_write(&mut self) -> Result<u64, bindings::StreamError> {
            Ok(4 * 1024 * 1024)
        }

        fn output_stream_write(&mut self, contents: Vec<u8>) -> Result<(), bindings::StreamError> {
            std::io::stderr().write_all(&contents).map_err(|err| {
                eprintln!("Unexpected error when writing to stdout: {err:?}");
                bindings::StreamError::Closed
            })
        }
    }
}
