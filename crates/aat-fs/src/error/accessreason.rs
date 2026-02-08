use std::path::PathBuf;

#[derive(Debug)]
pub enum AccessErrorReason {
    OpenFileAtPath(PathBuf),

    MoveToEndOfFile(PathBuf),

    SetSeekPosAtOffset(PathBuf, u64),

    ReadExactNumberOfBytesFromOffset {
        path: PathBuf,
        offset: u64,
        to_read: u64,
    },
}
