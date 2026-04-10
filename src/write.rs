use async_trait::async_trait;
use chrono::{DateTime, Local};
use std::fmt::Debug;
use std::io;
use std::path::Path;
#[allow(unused_imports)]
use tokio::io::{AsyncRead, AsyncSeek, AsyncWrite};

use crate::{DataReadSeek, VfsReader};

/// Writable virtual filesystem interface.
///
/// This trait extends read-only access with mutation operations such as
/// creating, deleting, modifying, and writing to files.
///
/// Implementors are expected to behave similarly to a typical POSIX-like
/// filesystem unless otherwise documented.
#[async_trait]
pub trait VfsWriter: Send + Sync + 'static + Debug + Unpin {
    /// Recursively remove a directory and all of its contents.
    ///
    /// # Arguments
    /// * `path` - Path to the directory to remove.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The directory does not exist
    /// * The path is not a directory
    /// * Permissions prevent deletion
    /// * Any contained file cannot be removed
    async fn remove_dir(&self, path: &Path) -> io::Result<()>;

    /// Remove a file.
    ///
    /// # Arguments
    /// * `path` - Path to the file to remove.
    ///
    /// # Notes
    /// * Must not remove directories.
    /// * If the path is a symlink, only the link itself is removed.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The file does not exist
    /// * The path refers to a directory
    /// * Permissions prevent deletion
    async fn remove_file(&self, path: &Path) -> io::Result<()>;

    /// Create a directory and any missing parent directories.
    ///
    /// # Arguments
    /// * `path` - Path to the directory to create.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The directory cannot be created
    /// * A non-directory file exists in the path
    /// * Permissions prevent creation
    async fn create_dir(&self, path: &Path) -> io::Result<()>;

    /// Set file timestamps.
    ///
    /// # Arguments
    /// * `path`  - Path to the file
    /// * `mtime` - Modified time
    /// * `atime` - Access time
    ///
    /// # Errors
    /// Returns an error if:
    /// * The file does not exist
    /// * The backend does not support timestamp updates
    /// * Permissions prevent modification
    async fn set_times(
        &self,
        path: &Path,
        mtime: DateTime<Local>,
        atime: DateTime<Local>,
    ) -> io::Result<()>;

    /// Set the file length.
    ///
    /// # Arguments
    /// * `path` - Path to the file
    /// * `size` - Desired file size in bytes
    ///
    /// # Notes
    /// * Existing files should be resized (truncate or extend).
    /// * Missing files should be created.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The file cannot be resized or created
    /// * Permissions prevent modification
    async fn set_length(&self, path: &Path, size: u64) -> io::Result<()>;

    /// Move or rename a file or directory.
    ///
    /// # Arguments
    /// * `old` - Source path
    /// * `new` - Destination path
    ///
    /// # Errors
    /// Returns an error if:
    /// * The source does not exist
    /// * The destination cannot be written
    /// * The move crosses unsupported boundaries (implementation-defined)
    async fn move_to(&self, old: &Path, new: &Path) -> io::Result<()>;

    /// Copy a file or directory.
    ///
    /// # Arguments
    /// * `old` - Source path
    /// * `new` - Destination path
    ///
    /// # Errors
    /// Returns an error if:
    /// * The source does not exist
    /// * The destination cannot be written
    /// * Copying is unsupported for the given type
    async fn copy_to(&self, old: &Path, new: &Path) -> io::Result<()>;

    /// Open a file for append-only writing.
    ///
    /// # Arguments
    /// * `item`      - Path to the file
    /// * `truncate`  - If true, existing content will be replaced if successfully opened.
    ///
    /// # Returns
    /// A writable stream positioned at the end of the file.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The file cannot be opened
    /// * Permissions prevent writing
    async fn open_write(&self, item: &Path, truncate: bool) -> io::Result<Box<dyn DataWrite>>;
}

/// Extended writable interface that supports random-access writes.
///
/// This is separated from [`VfsWriter`] because not all backends can support
/// seeking writes efficiently or at all.
#[async_trait]
pub trait VfsSeekWriter: VfsWriter {
    /// Open a file for random-access writing.
    ///
    /// # Arguments
    /// * `item` - Path to the file
    ///
    /// # Returns
    /// A writable + seekable stream.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The file cannot be opened
    /// * Random-access writes are unsupported
    /// * Permissions prevent writing
    async fn open_write_seek(&self, item: &Path) -> io::Result<Box<dyn DataWriteSeek>>;
}

/// Full filesystem interface combining read and advanced write capabilities.
///
/// This trait requires support for both reading and random-access writing.
#[async_trait]
pub trait VfsFull: VfsReader + VfsSeekWriter {
    /// Attempt to open a file for full (read + write + seek) access.
    ///
    /// # Arguments
    /// * `item` - Path to the file
    ///
    /// # Returns
    /// A [`DataFull`] handle.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The file cannot be opened
    /// * Permissions prevent access
    async fn open_full_seek(&self, item: &Path) -> io::Result<Box<dyn DataFull>>;
}

/// Writable stream abstraction.
///
/// This represents a sink for writing bytes asynchronously.
#[async_trait]
pub trait DataWrite: AsyncWrite + Send + Sync + 'static + Debug + Unpin {
    /// Finalize and close the stream.
    ///
    /// This should flush buffers and ensure all data is committed.
    ///
    /// # Errors
    /// Returns an error if:
    /// * Data cannot be flushed
    /// * The stream cannot be cleanly closed
    async fn close(&mut self) -> io::Result<()>;
}

/// Writable + seekable stream abstraction.
#[async_trait]
pub trait DataWriteSeek: DataWrite + AsyncSeek {}

/// Fully capable data stream supporting [`AsyncRead`], [`AsyncWrite`], and [`AsyncSeek`].
#[async_trait]
pub trait DataFull: DataReadSeek + DataWriteSeek {}
