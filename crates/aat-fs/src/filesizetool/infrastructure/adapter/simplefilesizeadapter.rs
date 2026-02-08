use super::super::super::domain::port::FileSizeFromFilesystemOutPort;
use crate::{Error, error::AccessErrorReason};
use std::{io::Seek, path::PathBuf};

pub struct SimpleFileSizeAdapter;

#[async_trait::async_trait]
impl FileSizeFromFilesystemOutPort for SimpleFileSizeAdapter {
    async fn determine_file_size(&self, path: PathBuf) -> Result<u64, Error> {
        let mut file = std::fs::File::options()
            .read(true)
            .open(&path)
            .map_err(|io_error| {
                Error::AccessError(AccessErrorReason::OpenFileAtPath(path.clone()), io_error)
            })?;

        file.seek(std::io::SeekFrom::End(0)).map_err(|io_error| {
            Error::AccessError(AccessErrorReason::MoveToEndOfFile(path.clone()), io_error)
        })
    }
}
