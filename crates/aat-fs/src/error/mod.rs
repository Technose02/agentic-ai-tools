use std::fmt::Display;

mod translator;
use adk_rust::serde_json;
pub use translator::{translate_de, translate_en};

mod accessreason;
pub use accessreason::AccessErrorReason;
mod validationreason;
pub use validationreason::ValidationErrorReason;
mod deserializereason;
pub use deserializereason::DeserializeReason;
mod serializereason;
pub use serializereason::SerializeReason;

#[derive(Debug)]
pub enum Error {
    AccessError(AccessErrorReason, std::io::Error),
    ValidationError(ValidationErrorReason),
    Deserialize(DeserializeReason, serde_json::Error),
    Serialize(SerializeReason, serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
impl core::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;
