use crate::domain::model::fileexiststool::InputParams;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct FileExistsParamsEn {
    // the filepath to check.
    pub path: String,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct FileExistsResultEn {
    // boolean value whether the file exists or not
    pub exists: bool,
}

impl TryFrom<FileExistsParamsEn> for InputParams {
    type Error = crate::error::Error;

    fn try_from(value: FileExistsParamsEn) -> Result<Self, Self::Error> {
        Ok(InputParams { path: value.path })
    }
}

impl From<bool> for FileExistsResultEn {
    fn from(value: bool) -> Self {
        FileExistsResultEn { exists: value }
    }
}
