use std::path::PathBuf;

#[derive(Debug)]
pub enum ValidationErrorReason {
    OffsetAtOrBehindEOF(PathBuf, u64),
    ReadBehindEOF {
        path: PathBuf,
        filesize: u64,
        target_position: u64,
    },
}
