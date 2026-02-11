use crate::{
    domain::{
        PinBoxedFuture,
        model::readchunktool::{InputParams, ResultContent},
    },
    error::Error,
};
use std::path::PathBuf;

pub trait ReadChunkToolInPort: Clone + Send + Sync + 'static {
    fn read_chunk(&self, params: InputParams) -> PinBoxedFuture<ResultContent, Error>;
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
