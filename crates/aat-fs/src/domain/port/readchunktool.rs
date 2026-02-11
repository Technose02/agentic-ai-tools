use crate::{
    PinBoxedFuture,
    domain::model::readchunktool::{InputParams, ResultContent},
    error::Error,
};
use std::path::PathBuf;

pub trait ReadChunkToolInPort: Clone + Send + Sync + 'static {
    fn read_chunk(&self, params: InputParams) -> PinBoxedFuture<ResultContent, Error>;
}

pub trait ReadChunkFromFilesystemOutPort: Sync + Send + 'static {
    fn read_chunk(
        &self,
        path: PathBuf,
        offset: u64,
        size: u64,
    ) -> PinBoxedFuture<ResultContent, Error>;
}
