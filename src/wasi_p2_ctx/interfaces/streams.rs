use std::io::Write;

use crate::*;

impl<T: WasiP2CtxHolder> crate::bindings::StreamsHost for T {
    fn input_stream_blocking_read(
        &mut self,
        _self_: WasiP2InputStreamResource,
        _len: u64,
    ) -> Result<Vec<u8>, bindings::StreamError> {
        todo!()
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
        match self_.id {
            0 | 1 => Ok(4 * 1024 * 1024),
            other => {
                eprintln!("Unknown output stream identifier: {other}");
                Err(bindings::StreamError::Closed)
            }
        }
    }

    fn output_stream_write(
        &mut self,
        self_: WasiP2OutputStreamResource,
        contents: Vec<u8>,
    ) -> Result<(), bindings::StreamError> {
        match self_.id {
            // TODO: better error handling
            0 => std::io::stdout().write_all(&contents).map_err(|err| {
                eprintln!("Unexpected error when writing to stdout: {err:?}");
                bindings::StreamError::Closed
            }),
            1 => std::io::stderr().write_all(&contents).map_err(|err| {
                eprintln!("Unexpected error when writing to stderr: {err:?}");
                bindings::StreamError::Closed
            }),
            other => {
                eprintln!("Unknown output stream identifier: {other}");
                Err(bindings::StreamError::Closed)
            }
        }
    }

    fn output_stream_blocking_flush(
        &mut self,
        self_: WasiP2OutputStreamResource,
    ) -> Result<(), bindings::StreamError> {
        match self_.id {
            0 | 1 => Ok(()),
            other => {
                eprintln!("Unknown output stream identifier: {other}");
                Err(bindings::StreamError::Closed)
            }
        }
    }

    fn output_stream_subscribe(
        &mut self,
        _self_: WasiP2OutputStreamResource,
    ) -> WasiP2PollableResource {
        WasiP2PollableResource {}
    }
}
