use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

/// Provides the root path for archive creation.
///
/// This trait defines the starting point (root directory) when building archives
/// from file system trees. For most path types, this is an empty path representing
/// the archive's root directory.
///
/// ```
/// use tree_to_archive::misc::ArchiveRoot;
///
/// let root: &str = ArchiveRoot::archive_root();
/// assert_eq!(root, "");
///
/// let root: String = ArchiveRoot::archive_root();
/// assert_eq!(root, "");
///
/// let root: std::path::PathBuf = ArchiveRoot::archive_root();
/// assert!(root.as_os_str().is_empty());
/// ```
pub trait ArchiveRoot: Sized {
    /// Returns the root path for archive construction.
    fn archive_root() -> Self;
}

impl ArchiveRoot for &str {
    fn archive_root() -> Self {
        ""
    }
}

impl ArchiveRoot for String {
    fn archive_root() -> Self {
        String::new()
    }
}

impl ArchiveRoot for &Path {
    fn archive_root() -> Self {
        "".as_ref()
    }
}

impl ArchiveRoot for PathBuf {
    fn archive_root() -> Self {
        PathBuf::new()
    }
}

impl<Path: ArchiveRoot> ArchiveRoot for Box<Path> {
    fn archive_root() -> Self {
        Box::new(Path::archive_root())
    }
}

impl<Path: ArchiveRoot> ArchiveRoot for Rc<Path> {
    fn archive_root() -> Self {
        Rc::new(Path::archive_root())
    }
}

impl<Path: ArchiveRoot> ArchiveRoot for Arc<Path> {
    fn archive_root() -> Self {
        Arc::new(Path::archive_root())
    }
}

impl<'a, Path> ArchiveRoot for Cow<'a, Path>
where
    Path: ToOwned + ?Sized,
    &'a Path: ArchiveRoot,
{
    fn archive_root() -> Self {
        Cow::Borrowed(<&Path>::archive_root())
    }
}
