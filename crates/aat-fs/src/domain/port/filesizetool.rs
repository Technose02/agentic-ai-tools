use crate::{
    domain::{DomainService, PinBoxedFuture, model::filesizetool::InputParams},
    error::Error,
};
use std::path::PathBuf;

pub trait FileSizeToolInPort: Clone + Send + Sync + 'static {
    fn determine_file_size(&self, params: InputParams) -> PinBoxedFuture<u64, Error>;
}

#[async_trait::async_trait]
pub trait FileSizeFromFilesystemOutPort: Sync + Send + 'static {
    async fn determine_file_size(&self, path: PathBuf) -> Result<u64, Error>;
}

impl<S> DomainService for S
where
    S: FileSizeToolInPort,
{
    type Error = Error;
    type Params = InputParams;
    type Result = u64;

    fn invoke(&self, params: Self::Params) -> PinBoxedFuture<Self::Result, Self::Error> {
        self.determine_file_size(params)
    }
}
