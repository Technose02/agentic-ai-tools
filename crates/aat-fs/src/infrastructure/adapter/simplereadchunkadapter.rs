use crate::{
    PinBoxedFuture,
    domain::{
        model::readchunktool::ResultContent, port::readchunktool::ReadChunkFromFilesystemOutPort,
    },
    error::Error,
};
use std::{
    io::{Read, Seek},
    path::PathBuf,
};

pub struct SimpleReadChunkAdapter;

impl ReadChunkFromFilesystemOutPort for SimpleReadChunkAdapter {
    fn read_chunk(
        &self,
        path: PathBuf,
        offset: u64,
        to_read: u64,
    ) -> PinBoxedFuture<ResultContent, Error> {
        Box::pin(async move {
            let mut buf = vec![0_u8; to_read as usize];
            let mut file = std::fs::File::options()
                .read(true)
                .open(&path)
                .map_err(|io_error| {
                    Error::Access(
                        crate::error::AccessErrorReason::OpenFileAtPath(path.clone()),
                        io_error,
                    )
                })?;

            file.seek(std::io::SeekFrom::Start(offset))
                .map_err(|io_error| {
                    Error::Access(
                        crate::error::AccessErrorReason::SetSeekPosAtOffset(path.clone(), offset),
                        io_error,
                    )
                })?;

            file.read_exact(&mut buf).map_err(|io_error| {
                Error::Access(
                    crate::error::AccessErrorReason::ReadExactNumberOfBytesFromOffset {
                        path,
                        offset,
                        to_read,
                    },
                    io_error,
                )
            })?;

            match String::from_utf8(buf) {
                Ok(text) => Ok(ResultContent::Text(text)),
                Err(e) => Ok(ResultContent::Binary(e.into_bytes())),
            }
        })
    }
}
