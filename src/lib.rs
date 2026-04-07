mod filters;
mod meta;
mod read;
mod write;

pub use filters::*;
pub use meta::*;
pub use read::*;
pub use write::*;

use async_trait::async_trait;
use futures_lite::Stream;
use std::io;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use uuid::Uuid;

pub type MetaStream = dyn Stream<Item = io::Result<Metadata>> + Send;

#[async_trait]
pub trait SizedQuery: Send + Sync {
    async fn size(self: Arc<Self>) -> io::Result<Option<u64>>;

    async fn stream(self: Arc<Self>) -> io::Result<Pin<Box<MetaStream>>>;
}

#[async_trait]
pub trait VfsBackend: Eq + PartialEq {
    /// Returns the ID of the VFS backend.
    fn id(&self) -> Uuid;

    /// Returns the name of the VFS backend.
    fn name(&self) -> &str;

    /// Convert a relative path into a backend-specific absolute path.
    fn realpath(&self, item: &Path) -> PathBuf;

    /// Attempts to upgrade to a [`VfsReader`].
    fn reader(self: Arc<Self>) -> Option<Arc<dyn VfsReader>>;

    /// Attempts to upgrade to a [`VfsWriter`].
    fn writer(self: Arc<Self>) -> Option<Arc<dyn VfsWriter>>;

    /// Retrieves usage information if applicable.
    async fn get_usage(&self) -> io::Result<Option<DataUsage>>;
}
