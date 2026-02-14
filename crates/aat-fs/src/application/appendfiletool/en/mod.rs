use crate::{
    application::{appendfiletool::create_append_file_tool, errortranslator::ErrorTranslatorEn},
    domain::port::appendfiletool::AppendFileToolInPort,
};
use adk_rust::tool::FunctionTool;
use std::sync::Arc;

mod model;
use model::AppendFileParamsEn;

pub fn create_append_file_tool_en<S>(service: S) -> Arc<FunctionTool>
where
    S: AppendFileToolInPort,
{
    create_append_file_tool::<AppendFileParamsEn, (), S, ErrorTranslatorEn>(
        "Appends text to a local file",
        service,
    )
}
