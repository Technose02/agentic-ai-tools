use crate::{
    domain::{
        PinBoxedFuture,
        model::filesizetool::InputParams,
        port::filesizetool::{FileSizeFromFilesystemOutPort, FileSizeToolInPort},
    },
    error::Error,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct FileSizeToolService(pub Arc<dyn FileSizeFromFilesystemOutPort>);

impl FileSizeToolInPort for FileSizeToolService {
    fn determine_file_size(&self, params: InputParams) -> PinBoxedFuture<u64, Error> {
        let adapter_impl = self.0.clone();

        Box::pin(async move { adapter_impl.determine_file_size(params.path).await })
    }
}
