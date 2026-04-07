use crate::filters::FilterOptions;
use crate::meta::Metadata;
use std::fmt::Debug;
use std::io;
use std::io::{Read, Seek};
use std::path::Path;
use std::sync::Arc;
use crate::blocking::SizedQueryCompat;

/// Read-only virtual filesystem interface.
///
/// Provides metadata access and directory listing functionality.
pub trait VfsReaderCompat: Send + Sync + 'static + Debug {
    /// Reads the path from the beginning.
    fn open_read_start(&self, item: &Path) -> io::Result<Box<dyn DataReadCompat>>;

    /// Reads the path and supporting seek.
    fn open_read_random(&self, item: &Path) -> io::Result<Option<Box<dyn DataReadSeekCompat>>>;

    /// Retrieve metadata for a path.
    ///
    /// # Returns
    /// `Some(metadata)` if the item exists, otherwise `None`.
    ///
    /// # Errors
    /// Returns an error if metadata cannot be retrieved.
    fn get_metadata(&self, item: &Path) -> io::Result<Option<Metadata>>;

    /// List directory contents.
    ///
    /// # Arguments
    /// * `recursive` - Whether to recurse into subdirectories.
    /// * `root` - Whether to show metadata of the root.
    ///
    /// # Errors
    /// Returns an error if the directory cannot be read.
    fn read_dir(
        &self,
        item: &Path,
        opts: Option<FilterOptions>,
        recursive: bool,
        include_root: bool,
    ) -> io::Result<Arc<dyn SizedQueryCompat>>;
}

pub trait DataReadCompat: Read + Send + Sync + 'static + Debug {}

pub trait DataReadSeekCompat: DataReadCompat + Seek {}
