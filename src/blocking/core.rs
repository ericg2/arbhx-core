use crate::blocking::{DataReadSeekCompat, DataWriteSeekCompat, VfsReaderCompat, VfsWriterCompat};
use crate::{DataUsage, Metadata, VfsReader, VfsWriter};
use async_trait::async_trait;
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

    /// Attempts to upgrade to a [`VfsFullCompat`].
    fn full(self: Arc<Self>) -> Option<Arc<dyn VfsFullCompat>>;

    /// Retrieves usage information if applicable.
    fn get_usage(&self) -> io::Result<Option<DataUsage>>;
}

pub trait VfsFullCompat: VfsReaderCompat + VfsWriterCompat {
    /// Attempts to open the path in full mode.
    fn open_full_random(&self, item: &Path) -> io::Result<Option<Box<dyn DataFullCompat>>>;
}

pub trait DataFullCompat: DataReadSeekCompat + DataWriteSeekCompat {}
