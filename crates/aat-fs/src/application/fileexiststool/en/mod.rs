use crate::{
    application::{errortranslator::ErrorTranslatorEn, fileexiststool::create_file_exists_tool},
    domain::port::fileexiststool::FileExistsToolInPort,
};
use adk_rust::tool::FunctionTool;
use std::sync::Arc;

mod model;
use model::{FileExistsParamsEn, FileExistsResultEn};

pub fn create_file_exists_tool_en<S>(service: S) -> Arc<FunctionTool>
where
    S: FileExistsToolInPort,
{
    create_file_exists_tool::<FileExistsParamsEn, FileExistsResultEn, S, ErrorTranslatorEn>(
        "Checks whether the provide path points at an existing file or not",
        service,
    )
}
