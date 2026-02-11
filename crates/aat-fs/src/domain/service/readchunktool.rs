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

/*
#[derive(Clone)]
pub struct SizeAwareReadChunkToolService {
    pub chunk_reader: Arc<dyn ReadChunkFromFilesystemOutPort>,
    pub filesize_determiner: Arc<dyn FileSizeFromFilesystemOutPort>,
}

#[async_trait::async_trait]
impl ReadChunkToolInPort for SizeAwareReadChunkToolService {
    async fn read_chunk(
        &self,
        params: super::model::InputParams,
    ) -> Result<super::model::ResultContent, Error> {
        let filesize = self
            .filesize_determiner
            .determine_file_size(params.path.clone())
            .await?;

        if params.offset >= filesize {
            return Err(Error::ValidationError(
                ValidationErrorReason::OffsetAtOrBehindEOF(params.path.clone(), params.offset),
            ));
        }

        if params.offset + params.to_read > filesize {
            return Err(Error::ValidationError(
                ValidationErrorReason::ReadBehindEOF {
                    path: params.path.clone(),
                    filesize,
                    target_position: params.offset + params.to_read,
                },
            ));
        }

        self.chunk_reader
            .read_chunk(params.path, params.offset, params.to_read)
            .await
    }
}
*/
