use crate::error::{
    AccessErrorReason, DeserializeReason, Error, SerializeReason, ValidationErrorReason,
};
use adk_rust::serde_json;

mod de;
pub use de::ErrorTranslatorDe;
mod en;
pub use en::ErrorTranslatorEn;

pub(crate) trait ErrorTranslator {
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
