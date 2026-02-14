use crate::{
    application::{appendfiletool::create_append_file_tool, errortranslator::ErrorTranslatorDe},
    domain::port::appendfiletool::AppendFileToolInPort,
};
use adk_rust::tool::FunctionTool;
use std::sync::Arc;

mod model;
use model::AppendFileParamsDe;

pub fn create_append_file_tool_de<S>(service: S) -> Arc<FunctionTool>
where
    S: AppendFileToolInPort,
{
    create_append_file_tool::<AppendFileParamsDe, (), S, ErrorTranslatorDe>(
        "Schreibt Text an das Ende einer Datei",
        service,
    )
}
