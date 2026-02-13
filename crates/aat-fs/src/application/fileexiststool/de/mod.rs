use crate::{
    application::{errortranslator::ErrorTranslatorDe, fileexiststool::create_file_exists_tool},
    domain::port::fileexiststool::FileExistsToolInPort,
};
use adk_rust::tool::FunctionTool;
use std::sync::Arc;

mod model;
use model::{FileExistsParamsDe, FileExistsResultDe};

pub fn create_file_exists_tool_de<S>(service: S) -> Arc<FunctionTool>
where
    S: FileExistsToolInPort,
{
    create_file_exists_tool::<FileExistsParamsDe, FileExistsResultDe, S, ErrorTranslatorDe>(
        "Prueft, ob der angegebene Pfad auf eine vorhandene Datei zeigt",
        service,
    )
}
