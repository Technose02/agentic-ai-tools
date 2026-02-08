use super::model::InputParams;
use crate::Error;
use std::path::PathBuf;

#[async_trait::async_trait]
pub trait FileSizeToolInPort: Clone + Send + Sync + 'static {
    async fn determine_file_size(&self, params: InputParams) -> Result<u64, Error>;
}

#[async_trait::async_trait]
pub trait FileSizeFromFilesystemOutPort: Sync + Send + 'static {
    async fn determine_file_size(&self, path: PathBuf) -> Result<u64, Error>;
}
