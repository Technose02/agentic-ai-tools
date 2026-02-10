use crate::{
    domain::port::filesizetool::FileSizeFromFilesystemOutPort,
    error::{AccessErrorReason, Error},
};
use std::{io::Seek, path::PathBuf};

pub struct SimpleFileSizeAdapter;

#[async_trait::async_trait]
impl FileSizeFromFilesystemOutPort for SimpleFileSizeAdapter {
    async fn determine_file_size(&self, path: PathBuf) -> Result<u64, Error> {
        let mut file = std::fs::File::options()
            .read(true)
            .open(&path)
            .map_err(|io_error| {
                Error::Access(AccessErrorReason::OpenFileAtPath(path.clone()), io_error)
            })?;

        file.seek(std::io::SeekFrom::End(0)).map_err(|io_error| {
            Error::Access(AccessErrorReason::MoveToEndOfFile(path.clone()), io_error)
        })
    }
}
