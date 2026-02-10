pub(crate) mod model;

use crate::{
    application::{errortranslator::ErrorTranslator, handler},
    domain::{model::filesizetool::InputParams, port::filesizetool::FileSizeToolInPort},
    error::Error,
};
use adk_rust::tool::FunctionTool;
use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;

mod de;
mod en;
pub use de::create_file_size_tool_de;
pub use en::create_file_size_tool_en;

fn create_file_size_tool<P, R, S, T>(description: &'static str, service: S) -> Arc<FunctionTool>
where
    P: TryInto<InputParams, Error = Error> + DeserializeOwned + Serialize + JsonSchema,
    R: From<u64> + Serialize,
    S: FileSizeToolInPort,
    T: ErrorTranslator + 'static,
{
    let handler = move |_ctx, args| {
        let service = service.clone();
        let error_translator = T::translate;

        async move { handler::<P, InputParams, R, u64, S, Error>(service, args, error_translator).await }
    };

    Arc::new(
        FunctionTool::new("file_size_tool", description, handler).with_parameters_schema::<P>(),
    )
}
