use crate::{
    application::{errortranslator::ErrorTranslatorEn, readchunktool::create_read_chunk_tool},
    domain::port::readchunktool::ReadChunkToolInPort,
};
use adk_rust::tool::FunctionTool;
use std::sync::Arc;

mod model;
use model::{ReadChunkParamsEn, ReadChunkResultEn};

pub fn create_read_chunk_tool_en<S>(service: S) -> Arc<FunctionTool>
where
    S: ReadChunkToolInPort,
{
    create_read_chunk_tool::<ReadChunkParamsEn, ReadChunkResultEn, S, ErrorTranslatorEn>(
        "Reads part of a local file and returns the corresponding chunk",
        service,
    )
}
