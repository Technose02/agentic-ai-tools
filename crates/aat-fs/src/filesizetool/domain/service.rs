use super::port::{FileSizeFromFilesystemOutPort, FileSizeToolInPort};
use crate::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct FileSizeToolService(pub Arc<dyn FileSizeFromFilesystemOutPort>);

#[async_trait::async_trait]
impl FileSizeToolInPort for FileSizeToolService {
    async fn determine_file_size(&self, params: super::model::InputParams) -> Result<u64, Error> {
        self.0.determine_file_size(params.path).await
    }
}
