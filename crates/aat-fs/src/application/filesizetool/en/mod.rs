use crate::{
    application::{errortranslator::ErrorTranslatorEn, filesizetool::create_file_size_tool},
    domain::port::filesizetool::FileSizeToolInPort,
};
use adk_rust::tool::FunctionTool;
use std::sync::Arc;

mod model;
use model::{FileSizeParamsEn, FileSizeResultEn};

pub fn create_file_size_tool_en<S>(service: S) -> Arc<FunctionTool>
where
    S: FileSizeToolInPort,
{
    create_file_size_tool::<FileSizeParamsEn, FileSizeResultEn, S, ErrorTranslatorEn>(
        "Determines the size of a local file",
        service,
    )
}
