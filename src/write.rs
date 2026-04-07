use std::fmt::Debug;
use std::io;
use std::path::Path;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use crate::DataAppend;

/// Writable virtual filesystem interface.
///
/// Extends [`VfsReader`] with mutation operations.
#[async_trait]
pub trait VfsWriter: Send + Sync + 'static + Debug + Unpin {
    /// Recursively remove a directory and all contents.
    ///
    /// # Errors
    /// Returns an error if removal fails.
    async fn remove_dir(&self, dirname: &Path) -> io::Result<()>;

    /// Remove a file.
    ///
    /// # Notes
    /// * Must not remove directories.
    /// * Must remove the symlink itself if applicable.
    ///
    /// # Errors
    /// Returns an error if removal fails.
    async fn remove_file(&self, filename: &Path) -> io::Result<()>;

    /// Create a directory and any missing parents.
    ///
    /// # Errors
    /// Returns an error if creation fails.
    async fn create_dir(&self, item: &Path) -> io::Result<()>;

    /// Set file timestamps if the file exists.
    ///
    /// # Errors
    /// Returns an error if timestamps cannot be applied.
    async fn set_times(
        &self,
        item: &Path,
        mtime: DateTime<Local>,
        atime: DateTime<Local>,
    ) -> io::Result<()>;

    /// Set file length.
    ///
    /// # Notes
    /// * Existing files should be resized.
    /// * Missing files should be created.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    async fn set_length(&self, item: &Path, size: u64) -> io::Result<()>;

    /// Move or rename a path.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    async fn move_to(&self, old: &Path, new: &Path) -> io::Result<()>;

    /// Copy a path.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    async fn copy_to(&self, old: &Path, new: &Path) -> io::Result<()>;

    /// Opens the specified path in append mode.
    async fn open_append(&self, item: &Path, truncate: bool) -> io::Result<Box<dyn DataAppend>>;
}