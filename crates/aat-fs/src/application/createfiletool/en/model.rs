use crate::domain::model::createfiletool::InputParams;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct CreateFileParamsEn {
    // the path of the file to create.
    pub path: String,
}

// Domain-Mapper

impl TryFrom<CreateFileParamsEn> for InputParams {
    type Error = crate::error::Error;

    fn try_from(value: CreateFileParamsEn) -> Result<Self, Self::Error> {
        Ok(InputParams {
            path: PathBuf::from(value.path),
        })
    }
}
