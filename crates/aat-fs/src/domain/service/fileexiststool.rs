use crate::domain::{
    model::fileexiststool::InputParams,
    port::fileexiststool::{FileExistsFromFilesystemOutPort, FileExistsToolInPort},
};
use std::{path::PathBuf, sync::Arc};

#[derive(Clone)]
pub struct FileExistsToolService(pub Arc<dyn FileExistsFromFilesystemOutPort>);

impl FileExistsToolInPort for FileExistsToolService {
    fn check_file_exists(
        &self,
        params: InputParams,
    ) -> crate::PinBoxedFuture<bool, crate::error::Error> {
        let path = PathBuf::from(params.path);
        let adapter = self.0.clone();
        Box::pin(async move { adapter.check_file_exists(path).await })
    }
}
