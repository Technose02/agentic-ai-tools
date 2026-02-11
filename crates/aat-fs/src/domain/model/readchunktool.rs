use std::path::PathBuf;
pub struct InputParams {
    pub path: PathBuf,
    pub offset: u64,
    pub to_read: u64,
}

pub enum ResultContent {
    Text(String),
    Binary(Vec<u8>),
}
