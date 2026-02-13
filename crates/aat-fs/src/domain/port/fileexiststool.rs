use crate::{PinBoxedFuture, domain::model::fileexiststool::InputParams, error::Error};
use std::path::PathBuf;

pub trait FileExistsToolInPort: Clone + Send + Sync + 'static {
    fn check_file_exists(&self, params: InputParams) -> PinBoxedFuture<bool, Error>;
}

pub trait FileExistsFromFilesystemOutPort: Sync + Send + 'static {
    fn check_file_exists(&self, path: PathBuf) -> PinBoxedFuture<bool, Error>;
}
