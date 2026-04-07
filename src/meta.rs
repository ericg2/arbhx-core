use bytesize::ByteSize;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

/// Represents Metadata for a returned file. This is a
/// data-only `struct`, with no operations as a result.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Metadata {
    pub(crate) path: PathBuf,
    pub(crate) is_dir: bool,
    pub(crate) mtime: Option<DateTime<Utc>>,
    pub(crate) atime: Option<DateTime<Utc>>,
    pub(crate) ctime: Option<DateTime<Utc>>,
    pub(crate) size: u64,
}

impl Metadata {
    /// # Returns
    /// The [`Path::file_name`] of this file.
    pub fn name(&self) -> &OsStr {
        self.path.file_name().unwrap_or_default()
    }

    /// # Returns
    /// The full, absolute [`Path`] of the node.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// # Returns
    /// If the [`Path`] is a directory.
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    /// # Returns
    /// If the [`Path`] is a file (not a directory).
    ///
    /// # Important
    /// This backend does <b>NOT support symbolic links</b>. As a result, this
    /// function is simply the inverse of [`is_dir`] - with no special processing.
    pub fn is_file(&self) -> bool {
        !self.is_dir
    }

    /// # Returns
    /// The last modified [`DateTime`] if supported.
    pub fn mtime(&self) -> Option<DateTime<Utc>> {
        self.mtime.clone()
    }

    /// # Returns
    /// The last accessed [`DateTime`] if supported.
    pub fn atime(&self) -> Option<DateTime<Utc>> {
        self.atime.clone()
    }

    /// # Returns
    /// The [`ByteSize`] of this node.
    pub fn size(&self) -> ByteSize {
        ByteSize(self.size)
    }

    /// Converts [`std::fs::Metadata`] into a valid [`Metadata`] struct.
    ///
    /// # Arguments
    /// * `path` - The [`Path`] to represent.
    /// * `meta` - The [`std::fs::Metadata`] to convert.
    ///
    /// # Returns
    /// A valid [`Metadata`] struct for the conversion.
    pub(crate) fn from_io(path: impl AsRef<Path>, meta: std::fs::Metadata) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            is_dir: meta.is_dir(),
            mtime: meta.modified().ok().map(|x| x.into()),
            atime: meta.accessed().ok().map(|x| x.into()),
            ctime: meta.created().ok().map(|x| x.into()),
            size: meta.len(),
        }
    }
}
