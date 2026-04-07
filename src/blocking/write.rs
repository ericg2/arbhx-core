use chrono::{DateTime, Local};
use std::fmt::Debug;
use std::io;
use std::io::{Seek, Write};
use std::path::Path;

/// Writable virtual filesystem interface.
///
/// Extends [`VfsReaderCompat`] with mutation operations.
pub trait VfsWriterCompat: Send + Sync + 'static + Debug {
    /// Recursively remove a directory and all contents.
    ///
    /// # Errors
    /// Returns an error if removal fails.
    fn remove_dir(&self, dirname: &Path) -> io::Result<()>;

    /// Remove a file.
    ///
    /// # Notes
    /// * Must not remove directories.
    /// * Must remove the symlink itself if applicable.
    ///
    /// # Errors
    /// Returns an error if removal fails.
    fn remove_file(&self, filename: &Path) -> io::Result<()>;

    /// Create a directory and any missing parents.
    ///
    /// # Errors
    /// Returns an error if creation fails.
    fn create_dir(&self, item: &Path) -> io::Result<()>;

    /// Set file timestamps if the file exists.
    ///
    /// # Errors
    /// Returns an error if timestamps cannot be applied.
    fn set_times(
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
    fn set_length(&self, item: &Path, size: u64) -> io::Result<()>;

    /// Move or rename a path.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    fn move_to(&self, old: &Path, new: &Path) -> io::Result<()>;

    /// Copy a path.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    fn copy_to(&self, old: &Path, new: &Path) -> io::Result<()>;

    /// Opens the specified path in append mode.
    fn open_write_append(
        &self,
        item: &Path,
        overwrite: bool,
    ) -> io::Result<Box<dyn DataWriteCompat>>;

    /// Opens the specified path in full mode if applicable.
    fn open_write_random(&self, item: &Path) -> io::Result<Option<Box<dyn DataWriteSeekCompat>>>;
}

pub trait DataWriteCompat: Write + Send + Sync + 'static + Debug {
    /// Finalize and close the stream.
    ///
    /// # Errors
    /// Returns an error if the stream cannot be properly finalized.
    fn close(&mut self) -> io::Result<()>;
}

pub trait DataWriteSeekCompat: DataWriteCompat + Seek {}
