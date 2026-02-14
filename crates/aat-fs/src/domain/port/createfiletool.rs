use crate::{PinBoxedFuture, domain::model::createfiletool::InputParams, error::Error};
use std::path::PathBuf;

pub trait CreateFileToolInPort: Clone + Send + Sync + 'static {
    fn create_new_file(&self, params: InputParams) -> PinBoxedFuture<(), Error>;
}

pub trait CreateFileFromFilesystemOutPort: Sync + Send + 'static {
    fn create_new_file(&self, path: PathBuf) -> PinBoxedFuture<(), Error>;
}
