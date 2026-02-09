use crate::error::{
    AccessErrorReason, DeserializeReason, Error, SerializeReason, ValidationErrorReason,
};
use adk_rust::{error::AdkError, serde_json};

mod de;
use de::ErrorTranslatorDe;
mod en;
use en::ErrorTranslatorEn;

trait ErrorTranslator {
    fn translate_accesserror(reason: AccessErrorReason, io_error: std::io::Error) -> String;

    fn translate_validationserror(reason: ValidationErrorReason) -> String;

    fn translate_deserialize(
        reason: DeserializeReason,
        serdejsonserror: serde_json::Error,
    ) -> String;

    fn translate_serialize(reason: SerializeReason, serdejsonserror: serde_json::Error) -> String;

    fn translate(error: crate::error::Error) -> String {
        match error {
            Error::AccessError(reason, ioerror) => Self::translate_accesserror(reason, ioerror),
            Error::ValidationError(reason) => Self::translate_validationserror(reason),
            Error::Deserialize(reason, serdejsonserror) => {
                Self::translate_deserialize(reason, serdejsonserror)
            }
            Error::Serialize(reason, serdejsonserror) => {
                Self::translate_serialize(reason, serdejsonserror)
            }
        }
    }
}

pub fn translate_de(error: Error) -> AdkError {
    AdkError::Tool(ErrorTranslatorDe::translate(error))
}

pub fn translate_en(error: Error) -> AdkError {
    AdkError::Tool(ErrorTranslatorEn::translate(error))
}
