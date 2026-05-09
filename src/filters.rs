/// Filtering rules applied when listing or querying filesystem entries.
///
/// `FilterOptions` allows callers to restrict results based on filename
/// patterns, ignore rules, and file size limits.
///
/// This is typically used in directory listings and search operations
/// to reduce result sets before traversal or streaming begins.
#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
pub struct FilterOptions {
    /// Glob patterns used to include or exclude files and directories.
    ///
    /// Multiple patterns may be provided. Matching behavior depends on the
    /// backend, but generally follows standard glob semantics.
    ///
    /// Examples:
    /// - `"*.txt"` → match all text files
    /// - `"src/**"` → match everything under `src/`
    pub globs: Vec<String>,

    /// Case-insensitive glob patterns.
    ///
    /// These behave like [`globs`] but ignore filename casing when matching.
    /// Useful on case-insensitive filesystems or when user input is not
    /// normalized.
    pub ignore_globs: Vec<String>,

    /// Additional ignore rules sourced from external files.
    ///
    /// Each entry is interpreted like a `.gitignore` file path, and its
    /// rules are applied during filtering.
    ///
    /// Multiple ignore files may be provided and combined.
    pub custom_ignore_files: Vec<String>,

    /// Maximum allowed file size in bytes.
    ///
    /// Files larger than this value will be excluded from results.
    ///
    /// If `None`, no size-based filtering is applied.
    pub exclude_larger_than: Option<u64>,
}
