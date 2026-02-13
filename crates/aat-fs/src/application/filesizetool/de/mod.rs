use crate::{
    application::{errortranslator::ErrorTranslatorDe, filesizetool::create_file_size_tool},
    domain::port::filesizetool::FileSizeToolInPort,
};
use adk_rust::tool::FunctionTool;
use std::sync::Arc;

mod model;
use model::{FileSizeParamsDe, FileSizeResultDe};

pub fn create_file_size_tool_de<S>(service: S) -> Arc<FunctionTool>
where
    S: FileSizeToolInPort,
{
    create_file_size_tool::<FileSizeParamsDe, FileSizeResultDe, S, ErrorTranslatorDe>(
        "Ermittelt die Groesse einer lokalen Datei",
        service,
    )
}
