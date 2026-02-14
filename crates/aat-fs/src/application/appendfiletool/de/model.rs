use crate::domain::model::appendfiletool::InputParams;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct AppendFileParamsDe {
    // Der Pfad zu der Datei, an deren Ende der Text geschrieben werden soll.
    pub path: String,

    // Der Text, der an das Ende der Datei geschrieben werden soll.
    pub text: String,
}

// Domain-Mapper

impl TryFrom<AppendFileParamsDe> for InputParams {
    type Error = crate::error::Error;

    fn try_from(value: AppendFileParamsDe) -> Result<Self, Self::Error> {
        Ok(InputParams {
            path: PathBuf::from(value.path),
            text: value.text,
        })
    }
}
