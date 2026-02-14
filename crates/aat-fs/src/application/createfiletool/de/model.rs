use crate::domain::model::createfiletool::InputParams;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct CreateFileParamsDe {
    // Der Pfad zu der Datei, die erstellt werden soll.
    pub path: String,
}

// Domain-Mapper

impl TryFrom<CreateFileParamsDe> for InputParams {
    type Error = crate::error::Error;

    fn try_from(value: CreateFileParamsDe) -> Result<Self, Self::Error> {
        Ok(InputParams {
            path: PathBuf::from(value.path),
        })
    }
}
