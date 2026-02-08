use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct ReadChunkParams {
    // Der Pfad zu der Datei, aus der ein Chunk gelesen werden soll.
    // Der Pfad ist relativ zum aktuellen Arbeitsverzeichnis.
    pub path: String,

    // Der Offset, ab dem aus der Datei gelesen werden soll.
    // Eine positive Ganzzahl oder 0.
    pub offset: u64,

    // Die Anzahl der Bytes, die aus der Datei ab dem Offset gelesen werden sollen.
    // Eine positive Ganzzahl.
    pub to_read: u64,
}

#[derive(JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResultContent {
    // Content basierend auf UTF-8 kodiertem Klartext
    Utf8String(String),
    // Content basierend auf bas64-kodierten Binärdaten
    Base64String(String),
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct ReadChunkResult {
    // Der Content des gelesenen Chunks
    pub content: ResultContent,
}
