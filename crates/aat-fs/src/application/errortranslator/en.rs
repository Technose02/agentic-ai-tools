use crate::error::{AccessErrorReason, DeserializeReason, SerializeReason, ValidationErrorReason};
pub struct ErrorTranslatorEn;

impl super::ErrorTranslator for ErrorTranslatorEn {
    fn translate_accesserror(reason: AccessErrorReason, io_error: std::io::Error) -> String {
        match reason {
            AccessErrorReason::OpenFileAtPath(path) => {
                format!("error opening file at path '{path:#?}': {io_error}")
            }

            AccessErrorReason::MoveToEndOfFile(path) => format!(
                "error seeking to end of file at path '{:#?}': {io_error}",
                path
            ),

            AccessErrorReason::SetSeekPosAtOffset(path, offset) => {
                format!("error seeking to position {offset} of file at '{path:#?}': {io_error}")
            }

            AccessErrorReason::ReadExactNumberOfBytesFromOffset {
                path,
                offset,
                to_read,
            } => format!(
                "error reading exactly {to_read} bytes of file at '{path:#?}' starting at position {offset}: {io_error}"
            ),
        }
    }

    fn translate_validationserror(reason: ValidationErrorReason) -> String {
        match reason {
            ValidationErrorReason::OffsetAtOrBehindEOF(path, offset) => format!(
                "offset {offset} lies behind or exactly at the end of the file at '{path:#?}'"
            ),

            ValidationErrorReason::ReadBehindEOF {
                path,
                filesize,
                target_position,
            } => format!(
                "the size of file at '{path:#?}' is only {filesize} bytes, so you cannot read from it up to position {target_position}",
            ),

            ValidationErrorReason::FileDoesNotExist(path) => {
                format!("The path '{path:#?}' to the file is invalid as that file does not exist.")
            }

            ValidationErrorReason::PathIsNotAFile(path) => format!(
                "The path '{path:#?}' to the file is invalid as it does not point at a file."
            ),
        }
    }

    fn translate_deserialize(
        reason: DeserializeReason,
        serdejsonserror: adk_rust::serde_json::Error,
    ) -> String {
        match reason {
            DeserializeReason::Parameters => {
                format!("error deserializing parameters: {serdejsonserror}")
            }
        }
    }

    fn translate_serialize(
        reason: SerializeReason,
        serdejsonserror: adk_rust::serde_json::Error,
    ) -> String {
        match reason {
            SerializeReason::Result => format!("error serializing result: {serdejsonserror}"),
        }
    }
}
