use crate::domain::model::appendfiletool::InputParams;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct AppendFileParamsEn {
    // the path of the file to be appended.
    pub path: String,

    // the text to append to the file .
    pub text: String,
}

// Domain-Mapper

impl TryFrom<AppendFileParamsEn> for InputParams {
    type Error = crate::error::Error;

    fn try_from(value: AppendFileParamsEn) -> Result<Self, Self::Error> {
        Ok(InputParams {
            path: PathBuf::from(value.path),
            text: value.text,
        })
    }
}
