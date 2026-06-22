use crate::*;

impl<T: WasiP2CtxHolder> crate::bindings::StreamsHost for T {
    fn input_stream_blocking_read(
        &mut self,
        self_: bindings::InputStream,
        len: u64,
    ) -> Result<Vec<u8>, bindings::StreamError> {
        todo!()
    }

    fn input_stream_subscribe(&mut self, self_: bindings::InputStream) -> bindings::Pollable {
        todo!()
    }

    fn output_stream_check_write(
        &mut self,
        self_: bindings::OutputStream,
    ) -> Result<u64, bindings::StreamError> {
        todo!()
    }

    fn output_stream_write(
        &mut self,
        self_: bindings::OutputStream,
        contents: Vec<u8>,
    ) -> Result<(), bindings::StreamError> {
        todo!()
    }

    fn output_stream_blocking_flush(
        &mut self,
        self_: bindings::OutputStream,
    ) -> Result<(), bindings::StreamError> {
        todo!()
    }

    fn output_stream_subscribe(&mut self, self_: bindings::OutputStream) -> bindings::Pollable {
        todo!()
    }
}
