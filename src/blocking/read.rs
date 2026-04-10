use crate::blocking::SizedQueryCompat;
use crate::filters::FilterOptions;
use crate::meta::Metadata;

use std::fmt::Debug;
use std::io;
use std::io::{Read, Seek};
use std::path::Path;
use std::sync::Arc;

/// A read-only virtual filesystem interface (blocking/synchronous version).
///
/// `VfsReaderCompat` mirrors the behavior of the async [`VfsReader`] trait,
/// but uses standard blocking I/O traits instead of async ones.
///
/// This trait abstracts over different storage backends (local disk,
/// in-memory, network, archives, etc.), exposing a consistent API for
/// reading files and querying directory structure.
///
/// # Usage
///
/// Prefer [`open_read_start`](VfsReaderCompat::open_read_start) for sequential
/// reads (e.g. streaming or hashing), and
/// [`open_read_seek`](VfsReaderCompat::open_read_seek) when you need to seek
/// — such as range requests or format parsers that jump around the file.
pub trait VfsReaderCompat: Send + Sync + 'static + Debug {
    /// Opens a file for sequential reading from the beginning.
    ///
    /// Returns a blocking reader without seek support. Prefer this over
    /// [`open_read_seek`](VfsReaderCompat::open_read_seek) when you don't need
    /// to seek, as implementations may avoid the overhead of seekable handles.
    ///
    /// # Arguments
    ///
    /// * `item` — Path to the file to open.
    ///
    /// # Errors
    ///
    /// Returns an error if the path does not exist or cannot be opened.
    fn open_read_start(&self, item: &Path) -> io::Result<Box<dyn DataReadCompat>>;

    /// Opens a file for random-access reading with seek support.
    ///
    /// Returns a reader that implements both [`Read`] and [`Seek`],
    /// suitable for use cases such as range requests, seeking media
    /// formats, or any parser that needs to jump to arbitrary offsets.
    ///
    /// # Arguments
    ///
    /// * `item` — Path to the file to open.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(reader))` if the backend supports seekable reads
    /// * `Ok(None)` if only sequential reads are supported
    ///
    /// # Errors
    ///
    /// Returns an error if the path does not exist or cannot be opened.
    fn open_read_seek(&self, item: &Path) -> io::Result<Option<Box<dyn DataReadSeekCompat>>>;

    /// Retrieves metadata for a path without opening it.
    ///
    /// Useful for existence checks, size queries, and distinguishing files
    /// from directories before deciding how to read them.
    ///
    /// # Arguments
    ///
    /// * `item` — Path to inspect.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(metadata))` — The path exists and metadata was read successfully.
    /// * `Ok(None)` — The path does not exist.
    /// * `Err(_)` — The path may or may not exist, but metadata could not be retrieved
    ///   (e.g. permission denied, I/O error).
    fn get_metadata(&self, item: &Path) -> io::Result<Option<Metadata>>;

    /// Lists the contents of a directory.
    ///
    /// Returns a [`SizedQueryCompat`] that provides both the matched entries
    /// and their total count, enabling pagination without a second query.
    ///
    /// # Arguments
    ///
    /// * `item` — Path to the directory to list.
    /// * `opts` — Optional filter to restrict results by name, extension,
    ///   type, or other criteria. Pass `None` to return all entries.
    /// * `recursive` — When `true`, descends into subdirectories and returns
    ///   their contents as well. When `false`, only immediate children
    ///   are returned.
    /// * `include_root` — When `true`, the entry for `item` itself is
    ///   included at the top of the results alongside its children.
    ///
    /// # Errors
    ///
    /// Returns an error if `item` is not a directory or cannot be read.
    fn list(
        &self,
        item: &Path,
        opts: Option<FilterOptions>,
        recursive: bool,
        include_root: bool,
    ) -> io::Result<Arc<dyn SizedQueryCompat>>;
}

/// A sequential blocking byte stream with no seek support.
///
/// Returned by [`VfsReaderCompat::open_read_start`]. Implementors provide a
/// forward-only view of a file's contents. Use this for streaming pipelines
/// where position tracking is unnecessary.
pub trait DataReadCompat: Read + Send + Sync + 'static + Debug {}

/// A seekable blocking byte stream.
///
/// Extends [`DataReadCompat`] with [`Seek`], allowing callers to jump to
/// arbitrary positions within a file. Returned by
/// [`VfsReaderCompat::open_read_seek`]. Use this when the read pattern is
/// non-sequential — for example, serving range requests or parsing
/// container formats that reference data by offset.
pub trait DataReadSeekCompat: DataReadCompat + Seek {}
