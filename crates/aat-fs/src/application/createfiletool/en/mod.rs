use crate::{
    application::{createfiletool::create_create_file_tool, errortranslator::ErrorTranslatorEn},
    domain::port::createfiletool::CreateFileToolInPort,
};
use adk_rust::tool::FunctionTool;
use std::sync::Arc;

mod model;
use model::CreateFileParamsEn;

pub fn create_create_file_tool_en<S>(service: S) -> Arc<FunctionTool>
where
    S: CreateFileToolInPort,
{
    create_create_file_tool::<CreateFileParamsEn, (), S, ErrorTranslatorEn>(
        "creates a new and empty file",
        service,
    )
}
