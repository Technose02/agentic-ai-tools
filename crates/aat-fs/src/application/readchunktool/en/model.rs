use crate::{
    domain::model::readchunktool::{InputParams, ResultContent},
    error::{Error, ValidationErrorReason},
};
use base64::{Engine, prelude::BASE64_STANDARD};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct ReadChunkParamsEn {
    // The path to the file from which a chunk is to be read.
    // The path is either absolute or relative to the current working dir.
    pub path: String,

    // The offset from which reading a chunk from the file should start.
    // A non negative integer or 0.
    pub offset: u64,

    // The number of bytes to read from the file starting at offset.
    // A non negative integer.
    pub to_read: u64,
}

#[derive(JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResultContentEn {
    // Content base on utf-8 encoded plain text
    Utf8String(String),
    // Content based on base64 encoded binary data
    Base64String(String),
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct ReadChunkResultEn {
    // The content of the chunk that has been read from the file
    pub content: ResultContentEn,
}

// Domain-Mapper

impl TryFrom<ReadChunkParamsEn> for InputParams {
    type Error = crate::error::Error;

    fn try_from(value: ReadChunkParamsEn) -> Result<Self, Self::Error> {
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

impl From<ResultContent> for ResultContentEn {
    fn from(value: ResultContent) -> Self {
        match value {
            ResultContent::Text(text) => ResultContentEn::Utf8String(text),
            ResultContent::Binary(bytes) => {
                ResultContentEn::Base64String(BASE64_STANDARD.encode(bytes))
            }
        }
    }
}

impl From<ResultContent> for ReadChunkResultEn {
    fn from(value: ResultContent) -> Self {
        ReadChunkResultEn {
            content: value.into(),
        }
    }
}
