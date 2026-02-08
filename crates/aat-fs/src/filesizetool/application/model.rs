use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct FileSizeParams {
    // Der Pfad zu der Datei, deren Groesse ermittelt werden soll.
    pub path: String,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct FileSizeResult {
    // Die ermittelte Groesse der Datei
    pub size: u64,
}
