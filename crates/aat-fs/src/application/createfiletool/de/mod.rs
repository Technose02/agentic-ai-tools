use crate::{
    application::{createfiletool::create_create_file_tool, errortranslator::ErrorTranslatorDe},
    domain::port::createfiletool::CreateFileToolInPort,
};
use adk_rust::tool::FunctionTool;
use std::sync::Arc;

mod model;
use model::CreateFileParamsDe;

pub fn create_create_file_tool_de<S>(service: S) -> Arc<FunctionTool>
where
    S: CreateFileToolInPort,
{
    create_create_file_tool::<CreateFileParamsDe, (), S, ErrorTranslatorDe>(
        "Erstellt eine neue, leere Datei",
        service,
    )
}
