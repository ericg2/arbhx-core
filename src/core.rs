use async_trait::async_trait;
use futures_lite::Stream;
use std::fmt::Debug;
use std::io;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use uuid::Uuid;

use crate::{DataUsage, Metadata, VfsFull, VfsReader, VfsSeekWriter, VfsWriter};

/// Stream type for yielding metadata results.
///
/// Each item represents a single filesystem entry or result produced by
/// a query operation.
pub type MetaStream = dyn Stream<Item = io::Result<Metadata>> + Send;

/// Trait representing a query that may optionally know its total size.
///
/// This is useful for directory listings, search results, or other
/// operations where:
/// - The total number of results *may* be known ahead of time
/// - Results can be streamed incrementally
///
/// Implementations should aim to be efficient:
/// - `size()` should be fast and avoid full scans if possible
/// - `stream()` should lazily produce results
#[async_trait]
pub trait SizedQuery: Send + Sync {
    /// Returns the total number of results if known.
    ///
    /// # Returns
    /// * `Ok(Some(count))` if the size is known
    /// * `Ok(None)` if the size cannot be determined efficiently
    ///
    /// # Errors
    /// Returns an error if the size cannot be computed due to backend failure.
    async fn size(self: Arc<Self>) -> io::Result<Option<u64>>;

    /// Returns a stream of metadata results.
    ///
    /// # Returns
    /// A pinned stream yielding [`Metadata`] entries.
    ///
    /// # Errors
    /// Returns an error if the stream cannot be created.
    ///
    /// # Notes
    /// * The stream should be lazy and not precompute all results.
    /// * Errors during iteration should be returned as stream items.
    async fn stream(self: Arc<Self>) -> io::Result<Pin<Box<MetaStream>>>;
}

/// Core backend trait for a virtual filesystem.
///
/// A `VfsBackend` represents a storage provider (e.g., local filesystem,
/// cloud storage, FTP, in-memory store) and exposes its capabilities
/// through optional trait upgrades.
///
/// Backends are capability-based:
/// - Not all backends support all operations
/// - Consumers must check for supported interfaces via the upgrade methods
///
/// # Capability Model
/// Each `*_()` method attempts to "upgrade" the backend into a more specific
/// interface:
/// - [`VfsReader`] → read-only access
/// - [`VfsWriter`] → basic write support
/// - [`VfsSeekWriter`] → random-access write support
/// - [`VfsFull`] → full read/write/seek support
///
/// Returning `None` indicates the capability is not supported.
///
/// # Thread Safety
/// Implementations must be safe to use across threads.
#[async_trait]
pub trait VfsBackend: Send + Sync + 'static + Debug + Unpin {
    /// Returns a unique identifier for this backend instance.
    ///
    /// This ID should remain stable for the lifetime of the backend and can
    /// be used for caching, tracking, or distinguishing multiple backends.
    fn id(&self) -> Uuid;

    /// Convert a virtual/relative path into a backend-specific absolute path.
    ///
    /// # Arguments
    /// * `item` - The virtual path provided by the caller
    ///
    /// # Returns
    /// A backend-specific path suitable for internal use.
    ///
    /// # Notes
    /// * This does not guarantee the path exists.
    /// * Implementations may normalize, prefix, or remap paths.
    fn realpath(&self, item: &Path) -> PathBuf;

    /// Attempts to upgrade this backend to a read-only interface.
    ///
    /// # Returns
    /// * `Some` if read operations are supported
    /// * `None` if the backend does not support reading
    fn reader(self: Arc<Self>) -> Option<Arc<dyn VfsReader>>;

    /// Attempts to upgrade this backend to a basic writable interface.
    ///
    /// # Returns
    /// * `Some` if write operations are supported
    /// * `None` if the backend does not support writing.
    ///
    /// # Notes
    /// This does not guarantee support for random-access writes.
    fn writer(self: Arc<Self>) -> Option<Arc<dyn VfsWriter>>;

    /// Attempts to upgrade this backend to a random-access writable interface.
    ///
    /// # Returns
    /// * `Some` if seekable writes are supported
    /// * `None` if only sequential writes (or no writes) are supported
    ///
    /// # Notes
    /// This is a stronger capability than [`VfsWriter`].
    fn writer_seek(self: Arc<Self>) -> Option<Arc<dyn VfsSeekWriter>>;

    /// Attempts to upgrade this backend to a full-featured interface.
    ///
    /// # Returns
    /// * `Some` if full read/write/seek functionality is supported
    /// * `None` otherwise
    ///
    /// # Notes
    /// This is the most complete capability level and typically implies
    /// support for all other interfaces.
    fn full(self: Arc<Self>) -> Option<Arc<dyn VfsFull>>;

    /// Retrieves storage usage information if available.
    ///
    /// # Returns
    /// * `Ok(Some(DataUsage))` if usage information is supported
    /// * `Ok(None)` if the backend does not provide usage data
    ///
    /// # Errors
    /// Returns an error if the usage query fails.
    async fn get_usage(&self) -> io::Result<Option<DataUsage>>;
}
