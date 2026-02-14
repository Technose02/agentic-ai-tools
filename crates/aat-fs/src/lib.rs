mod application;
mod domain;
mod error;
mod infrastructure;

pub use application::appendfiletool::{create_append_file_tool_de, create_append_file_tool_en};
pub use application::createfiletool::{create_create_file_tool_de, create_create_file_tool_en};
pub use application::fileexiststool::{create_file_exists_tool_de, create_file_exists_tool_en};
pub use application::filesizetool::{create_file_size_tool_de, create_file_size_tool_en};
pub use application::readchunktool::{create_read_chunk_tool_de, create_read_chunk_tool_en};
pub mod service {
    pub use crate::domain::service::*;
}
pub mod adapter {
    pub use crate::infrastructure::adapter::*;
}

pub type PinBoxedFuture<R, E> =
    std::pin::Pin<Box<dyn std::future::Future<Output = std::result::Result<R, E>> + Send>>;
