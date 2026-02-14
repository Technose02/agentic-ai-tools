use crate::{
    PinBoxedFuture,
    domain::port::filesizetool::FileSizeFromFilesystemOutPort,
    error::{AccessErrorReason, Error},
};
use std::{io::Seek, path::PathBuf};

pub struct SimpleFileSizeAdapter;

impl FileSizeFromFilesystemOutPort for SimpleFileSizeAdapter {
    fn determine_file_size(&self, path: PathBuf) -> PinBoxedFuture<u64, Error> {
        Box::pin(async move {
            let mut file = std::fs::File::options()
                .read(true)
                .open(&path)
                .map_err(|io_error| {
                    Error::Access(
                        AccessErrorReason::OpenFileForReading(path.clone()),
                        io_error,
                    )
                })?;

            file.seek(std::io::SeekFrom::End(0)).map_err(|io_error| {
                Error::Access(AccessErrorReason::MoveToEndOfFile(path.clone()), io_error)
            })
        })
    }
}
