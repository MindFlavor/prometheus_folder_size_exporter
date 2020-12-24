use crate::folder_scanner::FolderWithSize;
use crate::Options;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct State {
    pub(crate) options: Options,
    pub(crate) shared_vec: Arc<RwLock<Vec<FolderWithSize>>>,
}

impl State {
    pub(crate) fn new(options: Options) -> Self {
        Self {
            options,
            shared_vec: Arc::new(RwLock::new(Vec::new())),
        }
    }
}
