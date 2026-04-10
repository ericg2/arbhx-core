use crate::SizedQuery;
use crate::filters::FilterOptions;
use crate::meta::Metadata;
use async_trait::async_trait;
use std::fmt::Debug;
use std::io;
use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncSeek};

/// A read-only virtual filesystem interface.
///
/// `VfsReader` abstracts over different storage backends (local disk, in-memory,
/// network, archives, etc.), exposing a consistent async API for reading files
/// and querying directory structure. Implementations must be cheaply cloneable
/// via `Arc` and safe to share across threads.
///
/// # Usage
///
/// Prefer [`open_read_start`](VfsReader::open_read_start) for sequential reads
/// (e.g. streaming or hashing), and [`open_read_random`](VfsReader::open_read_seek)
/// when you need to seek — such as range requests or format parsers that jump
/// around the file.
#[async_trait]
pub trait VfsReader: Send + Sync + 'static + Debug + Unpin {
    /// Opens a file for sequential reading from the beginning.
    ///
    /// Returns a streaming reader without seek support. Prefer this over
    /// [`open_read_seek`](VfsReader::open_read_seek) when you don't need
    /// to seek, as implementations may avoid the overhead of seekable handles.
    ///
    /// # Arguments
    ///
    /// * `item` — Path to the file to open.
    ///
    /// # Errors
    ///
    /// Returns an error if the path does not exist or cannot be opened.
    async fn open_read_start(&self, item: &Path) -> io::Result<Box<dyn DataRead>>;

    /// Opens a file for random-access reading with seek support.
    ///
    /// Returns a reader that implements both [`AsyncRead`] and [`AsyncSeek`],
    /// suitable for use cases such as HTTP range requests, seeking media
    /// formats, or any parser that needs to jump to arbitrary offsets.
    ///
    /// # Arguments
    ///
    /// * `item` — Path to the file to open.
    ///
    /// # Errors
    ///
    /// Returns an error if the path does not exist or cannot be opened.
    async fn open_read_seek(&self, item: &Path) -> io::Result<Box<dyn DataReadSeek>>;

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
    async fn get_metadata(&self, item: &Path) -> io::Result<Option<Metadata>>;

    /// Lists the contents of a directory.
    ///
    /// Returns a [`SizedQuery`] that provides both the matched entries and
    /// their total count, enabling pagination without a second query.
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
    async fn list(
        &self,
        item: &Path,
        opts: Option<FilterOptions>,
        recursive: bool,
        include_root: bool,
    ) -> io::Result<Arc<dyn SizedQuery>>;
}

/// A sequential async byte stream with no seek support.
///
/// Returned by [`VfsReader::open_read_start`]. Implementors provide a
/// forward-only view of a file's contents. Use this for streaming pipelines
/// where position tracking is unnecessary.
pub trait DataRead: AsyncRead + Send + Sync + 'static + Unpin + Debug {}

/// A seekable async byte stream.
///
/// Extends [`DataRead`] with [`AsyncSeek`], allowing callers to jump to
/// arbitrary positions within a file. Returned by
/// [`VfsReader::open_read_seek`]. Use this when the read pattern is
/// non-sequential — for example, serving HTTP range requests or parsing
/// container formats that reference data by offset.
pub trait DataReadSeek: DataRead + AsyncSeek {}
