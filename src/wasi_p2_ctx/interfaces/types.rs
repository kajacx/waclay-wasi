use crate::*;

impl<T: AsWasiP2Ctx> crate::bindings::TypesHost for T {
    fn descriptor_read_via_stream(
        &mut self,
        _self_: WasiP2DescriptorResource,
        _offset: bindings::Filesize,
    ) -> anyhow::Result<Result<crate::WasiP2InputStreamResource, bindings::ErrorCode>> {
        Ok(Err(bindings::ErrorCode::Access))
    }

    fn descriptor_write_via_stream(
        &mut self,
        _self_: WasiP2DescriptorResource,
        _offset: bindings::Filesize,
    ) -> anyhow::Result<Result<crate::WasiP2OutputStreamResource, bindings::ErrorCode>> {
        Ok(Err(bindings::ErrorCode::Access))
    }

    fn descriptor_append_via_stream(
        &mut self,
        _self_: WasiP2DescriptorResource,
    ) -> anyhow::Result<Result<crate::WasiP2OutputStreamResource, bindings::ErrorCode>> {
        Ok(Err(bindings::ErrorCode::Access))
    }

    fn descriptor_get_flags(
        &mut self,
        _self_: WasiP2DescriptorResource,
    ) -> anyhow::Result<Result<bindings::DescriptorFlags, bindings::ErrorCode>> {
        Ok(Err(bindings::ErrorCode::Access))
    }

    fn descriptor_create_directory_at(
        &mut self,
        _self_: crate::WasiP2DescriptorResource,
        _path: String,
    ) -> anyhow::Result<Result<(), bindings::ErrorCode>> {
        Ok(Err(bindings::ErrorCode::Access))
    }

    fn descriptor_stat(
        &mut self,
        _self_: WasiP2DescriptorResource,
    ) -> anyhow::Result<Result<bindings::DescriptorStat, bindings::ErrorCode>> {
        Ok(Err(bindings::ErrorCode::Access))
    }

    fn descriptor_stat_at(
        &mut self,
        _self_: crate::WasiP2DescriptorResource,
        _path_flags: bindings::PathFlags,
        _path: String,
    ) -> anyhow::Result<Result<bindings::DescriptorStat, bindings::ErrorCode>> {
        Ok(Err(bindings::ErrorCode::Access))
    }

    fn descriptor_open_at(
        &mut self,
        _self_: WasiP2DescriptorResource,
        _path_flags: bindings::PathFlags,
        _path: String,
        _open_flags: bindings::OpenFlags,
        _flags: bindings::DescriptorFlags,
    ) -> anyhow::Result<Result<WasiP2DescriptorResource, bindings::ErrorCode>> {
        Ok(Err(bindings::ErrorCode::Access))
    }

    fn descriptor_metadata_hash(
        &mut self,
        _self_: WasiP2DescriptorResource,
    ) -> anyhow::Result<Result<bindings::MetadataHashValue, bindings::ErrorCode>> {
        Ok(Err(bindings::ErrorCode::Access))
    }

    fn descriptor_metadata_hash_at(
        &mut self,
        _self_: crate::WasiP2DescriptorResource,
        _path_flags: bindings::PathFlags,
        _path: String,
    ) -> anyhow::Result<Result<bindings::MetadataHashValue, bindings::ErrorCode>> {
        Ok(Err(bindings::ErrorCode::Access))
    }
}
