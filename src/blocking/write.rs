use crate::blocking::{DataReadSeekCompat, VfsReaderCompat};
use chrono::{DateTime, Local};
use std::fmt::Debug;
use std::io;
use std::io::{Seek, Write};
use std::path::Path;

/// Writable virtual filesystem interface (blocking/synchronous version).
///
/// This trait mirrors the async [`VfsWriter`] interface but uses blocking
/// I/O primitives instead of async operations.
///
/// It extends read-only filesystem access with mutation capabilities such as:
/// - file and directory creation/removal
/// - metadata modification
/// - file copying and moving
/// - opening writable streams
///
/// Implementations should behave similarly to POSIX-like filesystems unless
/// otherwise specified.
pub trait VfsWriterCompat: Send + Sync + 'static + Debug {
    /// Recursively remove a directory and all its contents.
    ///
    /// # Arguments
    /// * `path` - Path to the directory to remove
    ///
    /// # Errors
    /// Returns an error if:
    /// - the directory does not exist
    /// - the path is not a directory
    /// - permissions prevent deletion
    /// - any child entry cannot be removed
    fn remove_dir(&self, path: &Path) -> io::Result<()>;

    /// Remove a file.
    ///
    /// # Arguments
    /// * `path` - Path to the file to remove
    ///
    /// # Notes
    /// - Must not remove directories
    /// - If the path is a symlink, only the link itself is removed
    ///
    /// # Errors
    /// Returns an error if:
    /// - the file does not exist
    /// - the path refers to a directory
    /// - permissions prevent deletion
    fn remove_file(&self, path: &Path) -> io::Result<()>;

    /// Create a directory and any missing parent directories.
    ///
    /// # Arguments
    /// * `path` - Path to the directory to create
    ///
    /// # Errors
    /// Returns an error if:
    /// - the directory cannot be created
    /// - a non-directory file exists in the path
    /// - permissions prevent creation
    fn create_dir(&self, path: &Path) -> io::Result<()>;

    /// Update file timestamps.
    ///
    /// # Arguments
    /// * `path` - Path to the file
    /// * `mtime` - Modified time
    /// * `atime` - Access time
    ///
    /// # Errors
    /// Returns an error if:
    /// - the file does not exist
    /// - timestamp updates are unsupported
    /// - permissions prevent modification
    fn set_times(
        &self,
        path: &Path,
        mtime: DateTime<Local>,
        atime: DateTime<Local>,
    ) -> io::Result<()>;

    /// Resize a file.
    ///
    /// # Arguments
    /// * `path` - Path to the file
    /// * `size` - New file size in bytes
    ///
    /// # Notes
    /// - If the file is larger, it should be truncated
    /// - If smaller, it should be extended (implementation-defined content)
    /// - If missing, it may be created depending on backend behavior
    ///
    /// # Errors
    /// Returns an error if:
    /// - resizing fails
    /// - permissions prevent modification
    fn set_length(&self, path: &Path, size: u64) -> io::Result<()>;

    /// Move or rename a file or directory.
    ///
    /// # Arguments
    /// * `old` - Source path
    /// * `new` - Destination path
    ///
    /// # Errors
    /// Returns an error if:
    /// - source does not exist
    /// - destination cannot be created
    /// - move crosses unsupported backend boundaries
    fn move_to(&self, old: &Path, new: &Path) -> io::Result<()>;

    /// Copy a file or directory.
    ///
    /// # Arguments
    /// * `old` - Source path
    /// * `new` - Destination path
    ///
    /// # Errors
    /// Returns an error if:
    /// - source does not exist
    /// - destination cannot be written
    /// - copying is unsupported for the given entry type
    fn copy_to(&self, old: &Path, new: &Path) -> io::Result<()>;

    /// Open a file for sequential writing.
    ///
    /// # Arguments
    /// * `path` - Path to the file
    /// * `truncate` - If true, existing content may be replaced
    ///
    /// # Returns
    /// A writable stream for appending or overwriting data.
    ///
    /// # Errors
    /// Returns an error if:
    /// - the file cannot be opened
    /// - writing is not permitted
    fn open_write(&self, path: &Path, truncate: bool) -> io::Result<Box<dyn DataWriteCompat>>;
}

/// Extended writable interface supporting random-access writes.
///
/// This mirrors the async [`VfsSeekWriter`] capability.
/// Some backends (e.g., object storage) may not support this feature.
pub trait VfsSeekWriterCompat: VfsWriterCompat {
    /// Open a file for random-access writing.
    ///
    /// # Arguments
    /// * `path` - Path to the file
    ///
    /// # Returns
    /// A writable + seekable stream.
    ///
    /// # Errors
    /// Returns an error if:
    /// - the file cannot be opened
    /// - permissions prevent writing
    fn open_write_seek(&self, path: &Path) -> io::Result<Box<dyn DataWriteSeekCompat>>;
}

/// Full filesystem interface combining read and advanced write capabilities.
///
/// This represents the highest capability level of a backend:
/// - read operations ([`VfsReaderCompat`])
/// - sequential writes ([`VfsWriterCompat`])
/// - seekable writes ([`VfsSeekWriterCompat`])
pub trait VfsFullCompat: VfsReaderCompat + VfsSeekWriterCompat {
    /// Open a file with full read/write/seek access if supported.
    ///
    /// # Arguments
    /// * `path` - Path to the file
    ///
    /// # Returns
    /// A [`DataFullCompat`] handle.
    ///
    /// # Errors
    /// Returns an error if:
    /// - the file cannot be opened
    /// - permissions prevent access
    fn open_full_seek(&self, path: &Path) -> io::Result<Box<dyn DataFullCompat>>;
}

/// Writable byte stream abstraction (blocking).
///
/// Represents a sink for writing data using blocking I/O.
///
/// Implementations must flush and close properly when `close` is called.
pub trait DataWriteCompat: Write + Send + Sync + 'static + Debug {
    /// Finalize and close the stream.
    ///
    /// This should ensure all buffered data is flushed and persisted.
    ///
    /// # Errors
    /// Returns an error if:
    /// - flushing fails
    /// - the stream cannot be cleanly closed
    fn close(&mut self) -> io::Result<()>;
}

/// Writable + seekable byte stream abstraction (blocking).
///
/// Extends [`DataWriteCompat`] with seek capability.
pub trait DataWriteSeekCompat: DataWriteCompat + Seek {}

/// Fully capable stream supporting read, write, and seek.
///
/// Used when a backend supports complete random-access file semantics.
pub trait DataFullCompat: DataReadSeekCompat + DataWriteSeekCompat {}
