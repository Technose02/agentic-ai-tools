use crate::{
    PinBoxedFuture,
    domain::{
        model::readchunktool::InputParams,
        model::readchunktool::ResultContent,
        port::{
            filesizetool::FileSizeFromFilesystemOutPort,
            readchunktool::{ReadChunkFromFilesystemOutPort, ReadChunkToolInPort},
        },
    },
    error::{Error, ValidationErrorReason},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct SizeAwareReadChunkToolService {
    pub chunk_reader: Arc<dyn ReadChunkFromFilesystemOutPort>,
    pub filesize_determiner: Arc<dyn FileSizeFromFilesystemOutPort>,
}

impl ReadChunkToolInPort for SizeAwareReadChunkToolService {
    fn read_chunk(&self, params: InputParams) -> PinBoxedFuture<ResultContent, Error> {
        let filesize_determiner = self.filesize_determiner.clone();
        let chunk_reader = self.chunk_reader.clone();

        Box::pin(async move {
            let filesize = filesize_determiner
                .determine_file_size(params.path.clone())
                .await?;

            if params.offset >= filesize {
                return Err(Error::Validation(
                    ValidationErrorReason::OffsetAtOrBehindEOF(params.path.clone(), params.offset),
                ));
            }

            if params.offset + params.to_read > filesize {
                return Err(Error::Validation(ValidationErrorReason::ReadBehindEOF {
                    path: params.path.clone(),
                    filesize,
                    target_position: params.offset + params.to_read,
                }));
            }

            chunk_reader
                .read_chunk(params.path, params.offset, params.to_read)
                .await
        })
    }
}
