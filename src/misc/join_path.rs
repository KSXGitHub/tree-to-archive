use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

/// A trait for joining two path-like values together.
///
/// This trait provides a unified interface for concatenating paths across different
/// string and path types, returning an owned version of the resulting path.
///
/// ```
/// use tree_to_archive::misc::JoinPath;
///
/// let base = "dir";
/// let joined: String = base.join("file.txt");
/// assert_eq!(joined, "dir/file.txt");
///
/// let base = String::from("parent/child");
/// let joined: String = base.join(&"grandchild".to_string());
/// assert_eq!(joined, "parent/child/grandchild");
/// ```
///
/// ```
/// use tree_to_archive::misc::JoinPath;
/// use std::path::{Path, PathBuf};
///
/// let joined: PathBuf = <Path as JoinPath>::join("/etc".as_ref(), "config".as_ref());
/// assert_eq!(joined, Path::new("/etc/config"));
///
/// let base = PathBuf::from("var/log");
/// let joined: PathBuf = <Path as JoinPath>::join("var/log".as_ref(), "app.log".as_ref());
/// assert_eq!(joined, Path::new("var/log/app.log"));
/// ```
pub trait JoinPath {
    /// The owned type returned by the join operation.
    type OwnedPath;
    /// Joins two path components, returning an owned path.
    fn join(&self, other: &Self) -> Self::OwnedPath;
}

impl JoinPath for str {
    type OwnedPath = String;
    fn join(&self, other: &Self) -> Self::OwnedPath {
        format!("{self}/{other}")
    }
}

impl JoinPath for String {
    type OwnedPath = String;
    fn join(&self, other: &Self) -> Self::OwnedPath {
        str::join(self, other)
    }
}

impl JoinPath for Path {
    type OwnedPath = PathBuf;
    fn join(&self, other: &Self) -> Self::OwnedPath {
        self.join(other)
    }
}

impl JoinPath for PathBuf {
    type OwnedPath = PathBuf;
    fn join(&self, other: &Self) -> Self::OwnedPath {
        Path::join(self, other)
    }
}

impl<Path: JoinPath + ?Sized> JoinPath for &Path {
    type OwnedPath = Path::OwnedPath;
    fn join(&self, other: &Self) -> Self::OwnedPath {
        Path::join(self, other)
    }
}

impl<Path: JoinPath + ?Sized> JoinPath for Box<Path> {
    type OwnedPath = Path::OwnedPath;
    fn join(&self, other: &Self) -> Self::OwnedPath {
        Path::join(self, other)
    }
}

impl<Path: JoinPath + ?Sized> JoinPath for Rc<Path> {
    type OwnedPath = Path::OwnedPath;
    fn join(&self, other: &Self) -> Self::OwnedPath {
        Path::join(self, other)
    }
}

impl<Path: JoinPath + ?Sized> JoinPath for Arc<Path> {
    type OwnedPath = Path::OwnedPath;
    fn join(&self, other: &Self) -> Self::OwnedPath {
        Path::join(self, other)
    }
}

impl<Path: JoinPath + ToOwned + ?Sized> JoinPath for Cow<'_, Path> {
    type OwnedPath = Path::OwnedPath;
    fn join(&self, other: &Self) -> Self::OwnedPath {
        Path::join(self, other)
    }
}
