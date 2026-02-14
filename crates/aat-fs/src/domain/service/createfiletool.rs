use crate::{
    PinBoxedFuture,
    domain::port::createfiletool::{CreateFileFromFilesystemOutPort, CreateFileToolInPort},
    error::Error,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct CreateFileToolService(pub Arc<dyn CreateFileFromFilesystemOutPort>);

impl CreateFileToolInPort for CreateFileToolService {
    fn create_new_file(
        &self,
        params: crate::domain::model::createfiletool::InputParams,
    ) -> PinBoxedFuture<(), Error> {
        let adapter_impl = self.0.clone();

        Box::pin(async move { adapter_impl.create_new_file(params.path).await })
    }
}
