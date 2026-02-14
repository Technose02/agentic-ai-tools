use crate::{PinBoxedFuture, domain::model::appendfiletool::InputParams, error::Error};
use std::path::PathBuf;

pub trait AppendFileToolInPort: Clone + Send + Sync + 'static {
    fn append_file(&self, params: InputParams) -> PinBoxedFuture<(), Error>;
}

pub trait AppendFileFromFilesystemOutPort: Sync + Send + 'static {
    fn append_file(&self, path: PathBuf, text: String) -> PinBoxedFuture<(), Error>;
}
