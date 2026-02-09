pub(crate) mod model;

use crate::{
    application::filesizetool::model::{
        FileSizeParamsDe, FileSizeParamsEn, FileSizeResultDe, FileSizeResultEn,
    },
    domain::{model::filesizetool::InputParams, port::filesizetool::FileSizeToolInPort},
    error::{DeserializeReason, Error, SerializeReason, translate_de, translate_en},
};
use adk_rust::{
    AdkError,
    serde_json::{self},
    tool::FunctionTool,
};
use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;

// TODO: Make this generic over the inport and related structs somehow

async fn handler<P, R>(
    service: impl FileSizeToolInPort,
    args: schemars::_serde_json::Value,
    error_translator: fn(Error) -> AdkError,
) -> Result<schemars::_serde_json::Value, AdkError>
where
    P: TryInto<InputParams, Error = Error> + DeserializeOwned,
    R: From<u64> + Serialize,
{
    // read, validate and map args
    let args = serde_json::value::from_value::<P>(args)
        .map_err(|e| error_translator(Error::Deserialize(DeserializeReason::Parameters, e)))?;
    let params: InputParams = args.try_into().map_err(error_translator)?;

    // invoke service
    let size = service
        .determine_file_size(params)
        .await
        .map_err(error_translator)?;

    // map result
    serde_json::value::to_value(R::from(size))
        .map_err(|e| error_translator(Error::Serialize(SerializeReason::Result, e)))
}

fn create_file_size_tool<P, R>(
    description: &'static str,
    service: impl FileSizeToolInPort,
    error_translator: fn(Error) -> AdkError,
) -> Arc<FunctionTool>
where
    P: TryInto<InputParams, Error = Error> + DeserializeOwned + Serialize + JsonSchema,
    R: From<u64> + Serialize,
{
    let handler = move |_ctx, args| {
        let service = service.clone();

        async move { handler::<P, R>(service, args, error_translator).await }
    };

    Arc::new(
        FunctionTool::new("file_size_tool", description, handler).with_parameters_schema::<P>(),
    )
}

fn create_file_size_tool_de(service: impl FileSizeToolInPort) -> Arc<FunctionTool> {
    create_file_size_tool::<FileSizeParamsDe, FileSizeResultDe>(
        "Ermittelt die Groesse einer lokalen Datei",
        service,
        translate_de,
    )
}

fn create_file_size_tool_en(service: impl FileSizeToolInPort) -> Arc<FunctionTool> {
    create_file_size_tool::<FileSizeParamsEn, FileSizeResultEn>(
        "Determines the size of a local file",
        service,
        translate_en,
    )
}
