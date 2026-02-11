use crate::{PinBoxedFuture, domain::model::filesizetool::InputParams, error::Error};
use std::path::PathBuf;

pub trait FileSizeToolInPort: Clone + Send + Sync + 'static {
    fn determine_file_size(&self, params: InputParams) -> PinBoxedFuture<u64, Error>;
}

pub trait FileSizeFromFilesystemOutPort: Sync + Send + 'static {
    fn determine_file_size(&self, path: PathBuf) -> PinBoxedFuture<u64, Error>;
}
