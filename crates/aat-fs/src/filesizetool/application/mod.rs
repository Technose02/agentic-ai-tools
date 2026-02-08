use super::domain::{model::InputParams, port::FileSizeToolInPort};
use crate::error::Error;
use adk_rust::{
    AdkError,
    serde_json::{self},
    tool::FunctionTool,
};
use std::{path::PathBuf, sync::Arc};

mod model;
use model::{FileSizeParams, FileSizeResult};

pub fn create_file_size_tool(
    service: impl FileSizeToolInPort,
    error_translator: fn(Error) -> AdkError,
) -> Arc<FunctionTool> {
    let handler = move |_ctx, args| {
        let service = service.clone();

        async move {
            // read, validate and map args
            let args = serde_json::value::from_value::<FileSizeParams>(args).map_err(|e| {
                AdkError::Tool(format!("Fehler beim Deserialisieren der Parameter: {e}"))
            })?;
            let input_params: InputParams = args.try_into()?;

            // execute service
            let size = service
                .determine_file_size(input_params)
                .await
                .map_err(error_translator)?;

            // map result

            serde_json::value::to_value(FileSizeResult { size }).map_err(|e| {
                adk_rust::AdkError::Tool(format!("Fehler beim Serialisieren des Ergebnisses: {e}"))
            })
        }
    };

    Arc::new(
        FunctionTool::new(
            "file_size_tool",
            "Ermittelt die Groesse einer Datei",
            handler,
        )
        .with_parameters_schema::<FileSizeParams>(),
    )
}

impl TryFrom<FileSizeParams> for InputParams {
    type Error = AdkError;

    fn try_from(value: FileSizeParams) -> Result<Self, Self::Error> {
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
        Ok(InputParams { path })
    }
}
