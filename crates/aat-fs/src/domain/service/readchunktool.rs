use crate::{
    PinBoxedFuture,
    domain::{
        model::readchunktool::{InputParams, ResultContent},
        port::readchunktool::{ReadChunkFromFilesystemOutPort, ReadChunkToolInPort},
    },
    error::Error,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct ReadChunkToolService(pub Arc<dyn ReadChunkFromFilesystemOutPort>);

impl ReadChunkToolInPort for ReadChunkToolService {
    fn read_chunk(&self, params: InputParams) -> PinBoxedFuture<ResultContent, Error> {
        let adapter_impl = self.0.clone();

        Box::pin(async move {
            adapter_impl
                .read_chunk(params.path, params.offset, params.to_read)
                .await
        })
    }
}
