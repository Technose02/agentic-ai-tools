use crate::domain::model::fileexiststool::InputParams;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct FileExistsParamsDe {
    // Der zu prüfende Dateipfad.
    pub path: String,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct FileExistsResultDe {
    // Boolscher Wert, ob die Datei existiert
    pub exists: bool,
}

impl TryFrom<FileExistsParamsDe> for InputParams {
    type Error = crate::error::Error;

    fn try_from(value: FileExistsParamsDe) -> Result<Self, Self::Error> {
        Ok(InputParams { path: value.path })
    }
}

impl From<bool> for FileExistsResultDe {
    fn from(value: bool) -> Self {
        FileExistsResultDe { exists: value }
    }
}
