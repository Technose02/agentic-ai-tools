use std::fmt::Display;

mod translator;
pub use translator::{translate_de, translate_en};

mod accessreason;
pub use accessreason::AccessErrorReason;
mod validationreason;
pub use validationreason::ValidationErrorReason;

#[derive(Debug)]
pub enum Error {
    AccessError(AccessErrorReason, std::io::Error),
    ValidationError(ValidationErrorReason),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
impl core::error::Error for Error {}
