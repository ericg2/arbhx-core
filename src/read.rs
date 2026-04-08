use crate::SizedQuery;
use crate::filters::FilterOptions;
use crate::meta::Metadata;
use async_trait::async_trait;
use std::fmt::Debug;
use std::io;
use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncSeek};

/// Read-only virtual filesystem interface.
///
/// Provides metadata access and directory listing functionality.
#[async_trait]
pub trait VfsReader: Send + Sync + 'static + Debug + Unpin {
    /// Reads the path from the beginning.
    async fn open_read_start(&self, item: &Path) -> io::Result<Box<dyn DataRead>>;

    /// Reads the path and supporting seek.
    async fn open_read_random(&self, item: &Path) -> io::Result<Option<Box<dyn DataReadSeek>>>;

    /// Retrieve metadata for a path.
    ///
    /// # Returns
    /// `Some(metadata)` if the item exists, otherwise `None`.
    ///
    /// # Errors
    /// Returns an error if metadata cannot be retrieved.
    async fn get_metadata(&self, item: &Path) -> io::Result<Option<Metadata>>;

    /// List directory contents.
    ///
    /// # Arguments
    /// * `recursive` - Whether to recurse into subdirectories.
    /// * `root` - Whether to show metadata of the root.
    ///
    /// # Errors
    /// Returns an error if the directory cannot be read.
    async fn read_dir(
        &self,
        item: &Path,
        opts: Option<FilterOptions>,
        recursive: bool,
        include_root: bool,
    ) -> io::Result<Arc<dyn SizedQuery>>;
}

pub trait DataRead: AsyncRead + Send + Sync + 'static + Unpin + Debug {}

pub trait DataReadSeek: DataRead + AsyncSeek {}
