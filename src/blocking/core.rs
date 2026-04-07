use crate::blocking::{VfsReaderCompat, VfsWriterCompat};
use crate::{DataUsage, Metadata};
use std::fmt::Debug;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use uuid::Uuid;

pub type MetaStream = dyn Iterator<Item = io::Result<Metadata>> + Send;

pub trait SizedQueryCompat: Send + Sync {
    fn size(self: Arc<Self>) -> io::Result<Option<u64>>;

    fn stream(self: Arc<Self>) -> io::Result<Box<MetaStream>>;
}

pub trait VfsBackendCompat: Send + Sync + Debug + 'static {
    /// Returns the ID of the VFS backend.
    fn id(&self) -> Uuid;

    /// Returns the name of the VFS backend.
    fn name(&self) -> &str;

    /// Convert a relative path into a backend-specific absolute path.
    fn realpath(&self, item: &Path) -> PathBuf;

    /// Attempts to upgrade to a [`VfsReader`].
    fn reader(self: Arc<Self>) -> Option<Arc<dyn VfsReaderCompat>>;

    /// Attempts to upgrade to a [`VfsWriter`].
    fn writer(self: Arc<Self>) -> Option<Arc<dyn VfsWriterCompat>>;

    /// Retrieves usage information if applicable.
    fn get_usage(&self) -> io::Result<Option<DataUsage>>;
}
