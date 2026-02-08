use super::domain::{
    model::{InputParams, ResultContent as DomainResultContent},
    port::ReadChunkToolInPort,
};
use crate::error::Error;
use adk_rust::{
    AdkError,
    serde_json::{self},
    tool::FunctionTool,
};
use base64::{Engine, prelude::BASE64_STANDARD};
use std::{path::PathBuf, sync::Arc};

mod model;
use model::{ReadChunkParams, ReadChunkResult, ResultContent};

pub fn create_read_chunk_tool(
    service: impl ReadChunkToolInPort,
    error_translator: fn(Error) -> AdkError,
) -> Arc<FunctionTool> {
    let handler = move |_ctx, args| {
        let service = service.clone();

        async move {
            // read, validate and map args
            let args = serde_json::value::from_value::<ReadChunkParams>(args)
                .map_err(|e| AdkError::Tool(format!("error deserializing params: {e}")))?;
            let input_params: InputParams = args.try_into()?;

            // execute service
            let content = service
                .read_chunk(input_params)
                .await
                .map_err(error_translator)?
                .into();

            // map result

            serde_json::value::to_value(ReadChunkResult { content })
                .map_err(|e| adk_rust::AdkError::Tool(format!("error serializing result: {e}")))
        }
    };

    Arc::new(
        FunctionTool::new(
            "read_chunk_tool",
            "Liest einen Teil einer Datei und gibt den Inhalt zurueck",
            handler,
        )
        .with_parameters_schema::<ReadChunkParams>(),
    )
}

impl TryFrom<ReadChunkParams> for InputParams {
    type Error = AdkError;

    fn try_from(value: ReadChunkParams) -> Result<Self, Self::Error> {
        let path = PathBuf::from(value.path);
        if !path.exists() {
            return Err(AdkError::Tool(
                "Der Pfad zu der Datei ist ungueltig, weil es die Datei nicht gibt.".into(),
            ));
        }
        if !path.exists() {
            return Err(AdkError::Tool(
                "Der Pfad zu der Datei ist ungueltig, weil er nicht auf eine Datei zeigt.".into(),
            ));
        }

        let offset = value.offset;

        let to_read = value.to_read;
        if to_read == 0 {
            return Err(AdkError::Tool(
                "size darf nicht 0 sein, da in diesem Fall ja nichts gelesen wuerde.".into(),
            ));
        }

        Ok(InputParams {
            path,
            offset,
            to_read,
        })
    }
}

impl From<DomainResultContent> for ResultContent {
    fn from(value: DomainResultContent) -> Self {
        match value {
            DomainResultContent::Text(text) => ResultContent::Utf8String(text),
            DomainResultContent::Binary(bytes) => {
                ResultContent::Base64String(BASE64_STANDARD.encode(bytes))
            }
        }
    }
}
