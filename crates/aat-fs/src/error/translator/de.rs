use std::path::PathBuf;

use crate::error::{
    DeserializeReason, SerializeReason, ValidationErrorReason, accessreason::AccessErrorReason,
};
pub struct ErrorTranslatorDe;

impl super::ErrorTranslator for ErrorTranslatorDe {
    fn translate_accesserror(reason: AccessErrorReason, io_error: std::io::Error) -> String {
        match reason {
            AccessErrorReason::OpenFileAtPath(path) => format!(
                "Fehler beim Versuch, die Datei unter Pfad '{path:#?}' zu oeffnen: {io_error}"
            ),

            AccessErrorReason::MoveToEndOfFile(path) => format!(
                "Fehler beim Versuch, die Leseposition ans Ende der Datei unter Pfad '{path:#?}' zu setzen: {io_error}"
            ),

            AccessErrorReason::SetSeekPosAtOffset(path, offset) => format!(
                "Fehler beim Versuch, für die Datei unter Pfad '{path:#?}' die Leseposition {offset} zu setzen: {io_error}"
            ),

            AccessErrorReason::ReadExactNumberOfBytesFromOffset {
                path,
                offset,
                to_read,
            } => format!(
                "Fehler beim Versuch, aus der Datei unter Pfad '{path:#?}' ab Leseposition {offset} exakt {to_read} Bytes zu lesen: {io_error}"
            ),
        }
    }

    fn translate_validationserror(reason: ValidationErrorReason) -> String {
        match reason {
            ValidationErrorReason::OffsetAtOrBehindEOF(path, offset) => format!(
                "Der offset {offset} liegt hinter oder exakt bei dem Ende der Datei unter Pfad '{path:#?}'"
            ),

            ValidationErrorReason::ReadBehindEOF {
                path,
                filesize,
                target_position,
            } => format!(
                "Die Datei unter Pfad '{path:#?}' ist nur {filesize} Bytes gross, es kann also nicht bis position {target_position} gelesen werden.",
            ),

            ValidationErrorReason::FileDoesNotExist(path) => format!(
                "Der Pfad '{path:#?}' zu der Datei ist ungueltig, weil es die Datei nicht gibt."
            ),

            ValidationErrorReason::PathIsNotAFile(path) => format!(
                "Der Pfad '{path:#?}' zu der Datei ist ungueltig, weil er nicht auf eine Datei zeigt."
            ),
        }
    }

    fn translate_deserialize(
        reason: DeserializeReason,
        serdejsonserror: adk_rust::serde_json::Error,
    ) -> String {
        match reason {
            DeserializeReason::Parameters => {
                format!("Fehler beim Deserialisieren der Parameter: {serdejsonserror}")
            }
        }
    }

    fn translate_serialize(
        reason: SerializeReason,
        serdejsonserror: adk_rust::serde_json::Error,
    ) -> String {
        match reason {
            SerializeReason::Result => {
                format!("Fehler beim Serialisieren des Ergebnisses: {serdejsonserror}")
            }
        }
    }
}
