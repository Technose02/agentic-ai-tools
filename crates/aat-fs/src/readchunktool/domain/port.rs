use super::model::{InputParams, ResultContent};
use crate::Error;
use std::path::PathBuf;

#[async_trait::async_trait]
pub trait ReadChunkToolInPort: Clone + Send + Sync + 'static {
    async fn read_chunk(&self, params: InputParams) -> Result<ResultContent, Error>;
}

#[async_trait::async_trait]
pub trait ReadChunkFromFilesystemOutPort: Sync + Send + 'static {
    async fn read_chunk(
        &self,
        path: PathBuf,
        offset: u64,
        size: u64,
    ) -> Result<ResultContent, Error>;
}
