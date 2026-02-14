use crate::{
    PinBoxedFuture,
    domain::port::appendfiletool::{AppendFileFromFilesystemOutPort, AppendFileToolInPort},
    error::Error,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppendFileToolService(pub Arc<dyn AppendFileFromFilesystemOutPort>);

impl AppendFileToolInPort for AppendFileToolService {
    fn append_file(
        &self,
        params: crate::domain::model::appendfiletool::InputParams,
    ) -> PinBoxedFuture<(), Error> {
        let adapter_impl = self.0.clone();

        Box::pin(async move { adapter_impl.append_file(params.path, params.text).await })
    }
}
