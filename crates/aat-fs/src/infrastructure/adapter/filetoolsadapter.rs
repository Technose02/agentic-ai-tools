use adk_rust::telemetry::info;

use crate::{
    domain::{
        model::readchunktool::ResultContent as ReadChunkResultContent,
        port::{
            appendfiletool::AppendFileFromFilesystemOutPort,
            createfiletool::CreateFileFromFilesystemOutPort,
            fileexiststool::FileExistsFromFilesystemOutPort,
            filesizetool::FileSizeFromFilesystemOutPort,
            readchunktool::ReadChunkFromFilesystemOutPort,
        },
    },
    error::{AccessErrorReason, Error},
};
use std::{
    io::{Read, Seek, Write},
    path::PathBuf,
};

pub struct FileToolsAdapter;

impl FileToolsAdapter {
    fn check_file_exists(&self, path: std::path::PathBuf) -> Result<bool, Error> {
        Ok(path.exists() && path.is_file())
    }

    fn determine_file_size(&self, path: std::path::PathBuf) -> Result<u64, Error> {
        let mut file = std::fs::File::options()
            .read(true)
            .open(&path)
            .map_err(|io_error| {
                Error::Access(
                    AccessErrorReason::OpenFileForReading(path.clone()),
                    io_error,
                )
            })?;

        file.seek(std::io::SeekFrom::End(0)).map_err(|io_error| {
            Error::Access(AccessErrorReason::MoveToEndOfFile(path.clone()), io_error)
        })
    }

    fn read_chunk(
        &self,
        path: PathBuf,
        offset: u64,
        to_read: u64,
    ) -> Result<ReadChunkResultContent, Error> {
        let mut buf = vec![0_u8; to_read as usize];
        let mut file = std::fs::File::options()
            .read(true)
            .open(&path)
            .map_err(|io_error| {
                Error::Access(
                    crate::error::AccessErrorReason::OpenFileForReading(path.clone()),
                    io_error,
                )
            })?;

        file.seek(std::io::SeekFrom::Start(offset))
            .map_err(|io_error| {
                Error::Access(
                    crate::error::AccessErrorReason::SetSeekPosAtOffset(path.clone(), offset),
                    io_error,
                )
            })?;

        file.read_exact(&mut buf).map_err(|io_error| {
            Error::Access(
                crate::error::AccessErrorReason::ReadExactNumberOfBytesFromOffset {
                    path,
                    offset,
                    to_read,
                },
                io_error,
            )
        })?;

        match String::from_utf8(buf) {
            Ok(text) => Ok(ReadChunkResultContent::Text(text)),
            Err(e) => Ok(ReadChunkResultContent::Binary(e.into_bytes())),
        }
    }

    fn create_file(&self, path: PathBuf) -> Result<(), Error> {
        _ = std::fs::File::options()
            .create_new(true)
            .write(true)
            .open(&path)
            .map_err(|e| Error::Access(AccessErrorReason::CreateNewFile(path), e))?;
        Ok(())
    }

    fn append_text_to_file(&self, path: PathBuf, text: String) -> Result<(), Error> {
        let mut file = std::fs::File::options()
            .append(true)
            .open(&path)
            .map_err(|e| Error::Access(AccessErrorReason::OpenFileForWriting(path.clone()), e))?;
        file.write(text.as_bytes())
            .map_err(|e| Error::Access(AccessErrorReason::WriteToFile(path), e))?;
        Ok(())
    }
}

impl FileExistsFromFilesystemOutPort for FileToolsAdapter {
    fn check_file_exists(
        &self,
        path: std::path::PathBuf,
    ) -> crate::PinBoxedFuture<bool, crate::error::Error> {
        let res = self.check_file_exists(path);
        Box::pin(async move { res })
    }
}

impl FileSizeFromFilesystemOutPort for FileToolsAdapter {
    fn determine_file_size(
        &self,
        path: PathBuf,
    ) -> crate::PinBoxedFuture<u64, crate::error::Error> {
        let res = self.determine_file_size(path);
        Box::pin(async move { res })
    }
}

impl ReadChunkFromFilesystemOutPort for FileToolsAdapter {
    fn read_chunk(
        &self,
        path: PathBuf,
        offset: u64,
        to_read: u64,
    ) -> crate::PinBoxedFuture<ReadChunkResultContent, Error> {
        let res = self.read_chunk(path, offset, to_read);
        Box::pin(async move { res })
    }
}

impl CreateFileFromFilesystemOutPort for FileToolsAdapter {
    fn create_new_file(&self, path: PathBuf) -> crate::PinBoxedFuture<(), Error> {
        info!("creating new file '{path:#?}'");
        let res = self.create_file(path);

        Box::pin(async move { res })
    }
}

impl AppendFileFromFilesystemOutPort for FileToolsAdapter {
    fn append_file(&self, path: PathBuf, text: String) -> crate::PinBoxedFuture<(), Error> {
        let res = self.append_text_to_file(path, text);
        Box::pin(async move { res })
    }
}
