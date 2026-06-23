use std::io::{Read, Write};

use crate::*;

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
        _self_: WasiP2OutputStreamResource,
    ) -> Result<u64, bindings::StreamError> {
        Ok(4 * 1024 * 1024)
    }

    fn output_stream_write(
        &mut self,
        self_: WasiP2OutputStreamResource,
        contents: Vec<u8>,
    ) -> Result<(), bindings::StreamError> {
        match self_ {
            // TODO: better error handling
            WasiP2OutputStreamResource::Stdout => {
                std::io::stdout().write_all(&contents).map_err(|err| {
                    eprintln!("Unexpected error when writing to stdout: {err:?}");
                    bindings::StreamError::Closed
                })
            }
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
