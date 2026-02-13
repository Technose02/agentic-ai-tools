use crate::{
    application::{errortranslator::ErrorTranslatorDe, readchunktool::create_read_chunk_tool},
    domain::port::readchunktool::ReadChunkToolInPort,
};
use adk_rust::tool::FunctionTool;
use std::sync::Arc;

mod model;
use model::{ReadChunkParamsDe, ReadChunkResultDe};

pub fn create_read_chunk_tool_de<S>(service: S) -> Arc<FunctionTool>
where
    S: ReadChunkToolInPort,
{
    create_read_chunk_tool::<ReadChunkParamsDe, ReadChunkResultDe, S, ErrorTranslatorDe>(
        "Liest einen Teil einer Datei und gibt den Inhalt zurueck",
        service,
    )
}
