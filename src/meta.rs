use chrono::{DateTime, Utc};
use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// File or directory metadata returned by a VFS backend.
///
/// `Metadata` is a read-only snapshot describing a filesystem entry at a
/// specific point in time. It does not provide any mutation capabilities.
///
/// This structure is designed to be lightweight and transferable across
/// different backend implementations (local filesystem, remote storage,
/// archives, etc.).
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Setters, Default,
)]
#[setters(prefix = "set_", into)]
pub struct Metadata {
    pub(crate) path: PathBuf,
    pub(crate) is_dir: bool,
    pub(crate) ctime: Option<DateTime<Utc>>,
    pub(crate) mtime: Option<DateTime<Utc>>,
    pub(crate) atime: Option<DateTime<Utc>>,
    pub(crate) size: u64,
}

/// Storage usage statistics for a VFS backend.
///
/// Represents capacity and utilization information when supported by the
/// underlying storage system.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct DataUsage {
    /// Total allocated capacity in bytes.
    pub max_bytes: u64,

    /// Currently used storage in bytes.
    pub used_bytes: u64,

    /// Remaining available storage in bytes.
    pub free_bytes: u64,
}

impl Metadata {
    /// Returns the final component of the path (file or directory name).
    ///
    /// If the path has no file name, returns an empty value.
    pub fn name(&self) -> &OsStr {
        self.path.file_name().unwrap_or_default()
    }

    /// Returns the full virtual path of the entry.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns `true` if this entry is a directory.
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    /// Returns `true` if this entry is a file.
    ///
    /// This is defined as the logical inverse of [`is_dir`].
    /// Symbolic links are not supported by this system.
    pub fn is_file(&self) -> bool {
        !self.is_dir
    }

    /// Returns the last modification time, if available.
    pub fn mtime(&self) -> Option<DateTime<Utc>> {
        self.mtime
    }

    /// Returns the last access time, if available.
    pub fn atime(&self) -> Option<DateTime<Utc>> {
        self.atime
    }

    /// Returns the size of the entry in bytes.
    ///
    /// For directories, this value is backend-defined and may not reflect
    /// actual disk usage.
    pub fn size(&self) -> u64 {
        self.size
    }
}
