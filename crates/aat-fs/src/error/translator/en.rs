use crate::error::{AccessErrorReason, ValidationErrorReason};
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
        }
    }
}
