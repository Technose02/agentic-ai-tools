use crate::{
    domain::model::filesizetool::InputParams,
    error::{Error, ValidationErrorReason},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct FileSizeParamsDe {
    // Der Pfad zu der Datei, deren Groesse ermittelt werden soll.
    pub path: String,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct FileSizeResultDe {
    // Die ermittelte Groesse der Datei
    pub size: u64,
}

impl TryFrom<FileSizeParamsDe> for InputParams {
    type Error = crate::error::Error;

    fn try_from(value: FileSizeParamsDe) -> Result<Self, Self::Error> {
        let path = PathBuf::from(value.path);
        if !path.exists() {
            return Err(Error::ValidationError(
                ValidationErrorReason::FileDoesNotExist(path),
            ));
        }
        if !path.is_file() {
            return Err(Error::ValidationError(
                ValidationErrorReason::PathIsNotAFile(path),
            ));
        }
        Ok(InputParams { path })
    }
}

impl From<u64> for FileSizeResultDe {
    fn from(value: u64) -> Self {
        FileSizeResultDe { size: value }
    }
}
