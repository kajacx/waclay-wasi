use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::TypesHost for T {
    fn descriptor_read_via_stream(
        &mut self,
        _self_: WasiP2DescriptorResource,
        _offset: bindings::Filesize,
    ) -> anyhow::Result<anyhow::Result<crate::WasiP2InputStreamResource, bindings::ErrorCode>> {
        Ok(Err(bindings::ErrorCode::Access))
    }

    fn descriptor_write_via_stream(
        &mut self,
        _self_: WasiP2DescriptorResource,
        _offset: bindings::Filesize,
    ) -> anyhow::Result<anyhow::Result<crate::WasiP2OutputStreamResource, bindings::ErrorCode>>
    {
        Ok(Err(bindings::ErrorCode::Access))
    }

    fn descriptor_append_via_stream(
        &mut self,
        _self_: WasiP2DescriptorResource,
    ) -> anyhow::Result<anyhow::Result<crate::WasiP2OutputStreamResource, bindings::ErrorCode>>
    {
        Ok(Err(bindings::ErrorCode::Access))
    }

    fn descriptor_get_flags(
        &mut self,
        _self_: WasiP2DescriptorResource,
    ) -> anyhow::Result<anyhow::Result<bindings::DescriptorFlags, bindings::ErrorCode>> {
        Ok(Err(bindings::ErrorCode::Access))
    }

    fn descriptor_stat(
        &mut self,
        _self_: WasiP2DescriptorResource,
    ) -> anyhow::Result<anyhow::Result<bindings::DescriptorStat, bindings::ErrorCode>> {
        Ok(Err(bindings::ErrorCode::Access))
    }

    fn descriptor_open_at(
        &mut self,
        _self_: WasiP2DescriptorResource,
        _path_flags: bindings::PathFlags,
        _path: String,
        _open_flags: bindings::OpenFlags,
        _flags: bindings::DescriptorFlags,
    ) -> anyhow::Result<anyhow::Result<WasiP2DescriptorResource, bindings::ErrorCode>> {
        Ok(Err(bindings::ErrorCode::Access))
    }

    fn descriptor_metadata_hash(
        &mut self,
        _self_: WasiP2DescriptorResource,
    ) -> anyhow::Result<anyhow::Result<bindings::MetadataHashValue, bindings::ErrorCode>> {
        Ok(Err(bindings::ErrorCode::Access))
    }
}
