use adk_rust::serde_json;
use std::fmt::Display;

mod accessreason;
pub use accessreason::AccessErrorReason;
mod validationreason;
pub use validationreason::ValidationErrorReason;
mod deserializereason;
pub use deserializereason::DeserializeReason;
mod serializereason;
pub use serializereason::SerializeReason;

pub trait TError: core::error::Error {
    fn from_serialize_error(error: serde_json::Error) -> Self;
    fn from_deserialize_error(error: serde_json::Error) -> Self;
}

#[derive(Debug)]
pub enum Error {
    Access(AccessErrorReason, std::io::Error),
    Validation(ValidationErrorReason),
    Deserialize(DeserializeReason, serde_json::Error),
    Serialize(SerializeReason, serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
impl core::error::Error for Error {}

impl TError for Error {
    fn from_deserialize_error(error: serde_json::Error) -> Self {
        Error::Deserialize(DeserializeReason::Parameters, error)
    }

    fn from_serialize_error(error: serde_json::Error) -> Self {
        Error::Serialize(SerializeReason::Result, error)
    }
}
