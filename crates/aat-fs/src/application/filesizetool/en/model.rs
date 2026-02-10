use crate::{
    domain::model::filesizetool::InputParams,
    error::{Error, ValidationErrorReason},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct FileSizeParamsEn {
    // The path to the file which the size is to be determined for.
    pub path: String,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct FileSizeResultEn {
    // The determined size of the file
    pub size: u64,
}

impl TryFrom<FileSizeParamsEn> for InputParams {
    type Error = crate::error::Error;

    fn try_from(value: FileSizeParamsEn) -> Result<Self, Self::Error> {
        let path = PathBuf::from(value.path);
        if !path.exists() {
            return Err(Error::Validation(ValidationErrorReason::FileDoesNotExist(
                path,
            )));
        }
        if !path.is_file() {
            return Err(Error::Validation(ValidationErrorReason::PathIsNotAFile(
                path,
            )));
        }
        Ok(InputParams { path })
    }
}

impl From<u64> for FileSizeResultEn {
    fn from(value: u64) -> Self {
        FileSizeResultEn { size: value }
    }
}
