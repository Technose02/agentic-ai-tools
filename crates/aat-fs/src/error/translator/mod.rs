use crate::error::{AccessErrorReason, Error, ValidationErrorReason};
use adk_rust::error::AdkError;

mod de;
use de::ErrorTranslatorDe;
mod en;
use en::ErrorTranslatorEn;

trait ErrorTranslator {
    fn translate_accesserror(reason: AccessErrorReason, io_error: std::io::Error) -> String;

    fn translate_validationserror(reason: ValidationErrorReason) -> String;

    fn translate(error: crate::error::Error) -> String {
        match error {
            Error::AccessError(reason, ioerror) => Self::translate_accesserror(reason, ioerror),
            Error::ValidationError(reason) => Self::translate_validationserror(reason),
        }
    }
}

pub fn translate_de(error: Error) -> AdkError {
    AdkError::Tool(ErrorTranslatorDe::translate(error))
}

pub fn translate_en(error: Error) -> AdkError {
    AdkError::Tool(ErrorTranslatorEn::translate(error))
}
