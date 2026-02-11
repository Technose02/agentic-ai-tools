use std::path::PathBuf;

#[derive(Debug)]
pub enum ValidationErrorReason {
    FileDoesNotExist(PathBuf),
    PathIsNotAFile(PathBuf),
    OffsetAtOrBehindEOF(PathBuf, u64),
    ReadIsNull,
    ReadBehindEOF {
        path: PathBuf,
        filesize: u64,
        target_position: u64,
    },
}
