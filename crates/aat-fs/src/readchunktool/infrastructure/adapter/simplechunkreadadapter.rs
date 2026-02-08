use super::super::super::domain::{model::ResultContent, port::ReadChunkFromFilesystemOutPort};
use crate::Error;
use std::{
    io::{Read, Seek},
    path::PathBuf,
};

pub struct SimpleReadChunkAdapter;

#[async_trait::async_trait]
impl ReadChunkFromFilesystemOutPort for SimpleReadChunkAdapter {
    async fn read_chunk(
        &self,
        path: PathBuf,
        offset: u64,
        to_read: u64,
    ) -> Result<ResultContent, Error> {
        let mut buf = vec![0_u8; to_read as usize];
        let mut file = std::fs::File::options()
            .read(true)
            .open(&path)
            .map_err(|io_error| {
                Error::AccessError(
                    crate::error::AccessErrorReason::OpenFileAtPath(path.clone()),
                    io_error,
                )
            })?;

        file.seek(std::io::SeekFrom::Start(offset))
            .map_err(|io_error| {
                Error::AccessError(
                    crate::error::AccessErrorReason::SetSeekPosAtOffset(path.clone(), offset),
                    io_error,
                )
            })?;

        file.read_exact(&mut buf).map_err(|io_error| {
            Error::AccessError(
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
    }
}
