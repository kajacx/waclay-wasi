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
        todo!()
    }

    fn output_stream_check_write(
        &mut self,
        _self_: WasiP2OutputStreamResource,
    ) -> Result<u64, bindings::StreamError> {
        todo!()
    }

    fn output_stream_write(
        &mut self,
        _self_: WasiP2OutputStreamResource,
        _contents: Vec<u8>,
    ) -> Result<(), bindings::StreamError> {
        todo!()
    }

    fn output_stream_blocking_flush(
        &mut self,
        _self_: WasiP2OutputStreamResource,
    ) -> Result<(), bindings::StreamError> {
        todo!()
    }

    fn output_stream_subscribe(
        &mut self,
        _self_: WasiP2OutputStreamResource,
    ) -> WasiP2PollableResource {
        todo!()
    }
}
