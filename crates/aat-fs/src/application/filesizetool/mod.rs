use crate::{
    application::errortranslator::ErrorTranslator,
    domain::{model::filesizetool::InputParams, port::filesizetool::FileSizeToolInPort},
    error::{Error, TError},
};
use adk_rust::{AdkError, tool::FunctionTool};
use schemars::{
    _serde_json::{from_value, to_value},
    JsonSchema,
};
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;

mod de;
mod en;
pub use de::create_file_size_tool_de;
pub use en::create_file_size_tool_en;

fn create_file_size_tool<P, R, S, T>(description: &'static str, service: S) -> Arc<FunctionTool>
where
    P: TryInto<InputParams, Error = Error> + JsonSchema + Serialize + DeserializeOwned,
    R: From<u64> + JsonSchema + Serialize,
    S: FileSizeToolInPort,
    T: ErrorTranslator + 'static,
{
    let handler = move |_ctx, args| {
        let service = service.clone();
        let error_translator = T::translate;

        async move {
            // parse JSON as application model
            let args = from_value::<P>(args)
                .map_err(|e| AdkError::Tool(error_translator(Error::from_deserialize_error(e))))?;

            // map application model to domain model
            let params = args
                .try_into()
                .map_err(|e| AdkError::Tool(error_translator(e)))?;

            // compute response with domain service
            let res = service
                .determine_file_size(params)
                .await
                .map_err(|e| AdkError::Tool(error_translator(e)))?;

            // map result to application model and serialize to JSON
            to_value(R::from(res))
                .map_err(|e| AdkError::Tool(error_translator(Error::from_serialize_error(e))))
        }
    };

    Arc::new(
        FunctionTool::new("file_size_tool", description, handler)
            .with_parameters_schema::<P>()
            .with_response_schema::<R>(),
    )
}
