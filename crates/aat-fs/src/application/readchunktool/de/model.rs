use crate::{
    domain::model::readchunktool::{InputParams, ResultContent},
    error::{Error, ValidationErrorReason},
};
use base64::{Engine, prelude::BASE64_STANDARD};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct ReadChunkParamsDe {
    // Der Pfad zu der Datei, aus der ein Chunk gelesen werden soll.
    // Der Pfad ist entweder absolut oder relativ zum aktuellen Arbeitsverzeichnis.
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
pub enum ResultContentDe {
    // Content basierend auf UTF-8 kodiertem Klartext
    Utf8String(String),
    // Content basierend auf base64-kodierten Binärdaten
    Base64String(String),
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct ReadChunkResultDe {
    // Der Content des gelesenen Chunks
    pub content: ResultContentDe,
}

// Domain-Mapper

impl TryFrom<ReadChunkParamsDe> for InputParams {
    type Error = crate::error::Error;

    fn try_from(value: ReadChunkParamsDe) -> Result<Self, Self::Error> {
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

        let offset = value.offset;
        let to_read = value.to_read;
        if to_read == 0 {
            return Err(Error::Validation(ValidationErrorReason::ReadIsNull));
        }

        Ok(InputParams {
            path,
            offset,
            to_read,
        })
    }
}

impl From<ResultContent> for ResultContentDe {
    fn from(value: ResultContent) -> Self {
        match value {
            ResultContent::Text(text) => ResultContentDe::Utf8String(text),
            ResultContent::Binary(bytes) => {
                ResultContentDe::Base64String(BASE64_STANDARD.encode(bytes))
            }
        }
    }
}

impl From<ResultContent> for ReadChunkResultDe {
    fn from(value: ResultContent) -> Self {
        ReadChunkResultDe {
            content: value.into(),
        }
    }
}
