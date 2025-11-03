use crate::{
    misc::{ArchiveRoot, JoinPath},
    sealed::{ArchiveBuilder, FsTree},
};
use build_fs_tree::{FileSystemTree, MergeableFileSystemTree};
use core::ops::Deref;
use pipe_trait::Pipe;
use std::{
    borrow::Cow,
    io::{self, Write},
    path::Path,
};

impl<Writer: Write> ArchiveBuilder for tar::Builder<Writer> {}

fn append_file_to_tar<Writer, FilePath, Data>(
    builder: &mut tar::Builder<Writer>,
    path: FilePath,
    data: Data,
) -> io::Result<()>
where
    Writer: Write,
    FilePath: AsRef<Path>,
    Data: AsRef<[u8]>,
{
    let data = data.as_ref();
    let mut header = tar::Header::new_gnu();
    header.set_size(data.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    builder.append_data(&mut header, path, data)
}

/// Appends in-memory filesystem trees to tar archive builders.
///
/// ```no_run
/// use tree_to_archive::{
///     tree::{FileSystemTree, dir, file},
///     tar::{AppendToTar, Builder},
/// };
/// # use std::io;
///
/// // Create a file system tree
/// let tree: FileSystemTree<&str, &str> = dir! {
///     "README.md" => file!("# Example Rust project"),
///     "src" => dir! {
///         "main.rs" => file!("fn main() {}")
///     }
/// };
///
/// // Append the tree to a tar archive
/// let mut buffer = Vec::new();
/// let mut builder = Builder::new(&mut buffer);
/// tree.append_to_tar(&mut builder, "".as_ref())?;
/// builder.finish()?;
/// # Ok::<(), io::Error>(())
/// ```
pub trait AppendToTar<Builder: ?Sized>: FsTree {
    /// Type of archive entry names.
    type Path: ?Sized;
    /// Appends the tree to a path within the archive.
    fn append_to_tar(&self, builder: &mut Builder, path: &Self::Path) -> io::Result<()>;
}

impl<Writer, FilePath, FileContent> AppendToTar<tar::Builder<Writer>>
    for FileSystemTree<FilePath, FileContent>
where
    Writer: Write,
    FilePath: Deref + Ord,
    FilePath::Target:
        AsRef<Path> + JoinPath + ToOwned<Owned = <FilePath::Target as JoinPath>::OwnedPath>,
    FileContent: AsRef<[u8]>,
{
    type Path = FilePath::Target;
    fn append_to_tar(
        &self,
        builder: &mut tar::Builder<Writer>,
        path: &Self::Path,
    ) -> io::Result<()> {
        match self {
            FileSystemTree::File(data) => append_file_to_tar(builder, path, data),
            FileSystemTree::Directory(children) => {
                for (suffix, subtree) in children {
                    let path: Cow<FilePath::Target> = match path.as_ref().to_str() {
                        Some("" | ".") => Cow::Borrowed(suffix),
                        _ => path.join(suffix).pipe(Cow::Owned),
                    };
                    subtree.append_to_tar(builder, path.as_ref())?;
                }
                Ok(())
            }
        }
    }
}

impl<Writer, FilePath, FileContent> AppendToTar<tar::Builder<Writer>>
    for MergeableFileSystemTree<FilePath, FileContent>
where
    Writer: Write,
    FilePath: Deref + Ord,
    FilePath::Target:
        AsRef<Path> + JoinPath + ToOwned<Owned = <FilePath::Target as JoinPath>::OwnedPath>,
    FileContent: AsRef<[u8]>,
{
    type Path = FilePath::Target;
    fn append_to_tar(
        &self,
        builder: &mut tar::Builder<Writer>,
        path: &Self::Path,
    ) -> io::Result<()> {
        AsRef::<FileSystemTree<FilePath, FileContent>>::as_ref(self).append_to_tar(builder, path)
    }
}

/// Appends in-memory filesystem trees to archive builders.
///
/// ```no_run
/// use tree_to_archive::{
///     tree::{FileSystemTree, dir, file},
///     tar::{AppendTree, Builder},
/// };
/// # use std::io;
///
/// // Create a file system tree
/// let tree: FileSystemTree<&str, &str> = dir! {
///     "README.md" => file!("# Example project"),
///     "src" => dir! {
///         "lib.rs" => file!("pub fn hello() {}")
///     }
/// };
///
/// // Append the tree to a tar archive using AppendTree
/// let mut buffer = Vec::new();
/// let mut builder = Builder::new(&mut buffer);
/// builder.append_tree(&tree, "project")?;
/// builder.finish()?;
/// # Ok::<(), io::Error>(())
/// ```
pub trait AppendTree<Tree>: ArchiveBuilder
where
    Tree: AppendToTar<Self> + ?Sized,
{
    /// Appends a tree to a path within this archive.
    fn append_tree(&mut self, tree: &Tree, path: &Tree::Path) -> io::Result<()>;
}

impl<Builder, Tree> AppendTree<Tree> for Builder
where
    Builder: ArchiveBuilder + ?Sized,
    Tree: AppendToTar<Builder> + ?Sized,
{
    fn append_tree(&mut self, tree: &Tree, path: &Tree::Path) -> io::Result<()> {
        tree.append_to_tar(self, path)
    }
}

/// Creates complete tar archives from in-memory filesystem tree.
///
/// ```no_run
/// use tree_to_archive::{
///     tree::{FileSystemTree, dir, file},
///     tar::BuildTar,
/// };
/// # use std::io;
///
/// let tree: FileSystemTree<&str, &str> = dir! {
///     "README.md" => file!("# My Project"),
///     "src" => dir! {
///         "lib.rs" => file!("pub fn greet() {}"),
///         "main.rs" => file!("fn main() { greeting::greet(); }"),
///     },
///     "Cargo.toml" => file!(r#"
///         [package]
///         name = "greeting"
///         version = "0.1.0"
///     "#),
/// };
///
/// let archive_data: Vec<u8> = tree.build_tar()?;
/// # Ok::<(), io::Error>(())
/// ```
pub trait BuildTar<Writer>: FsTree {
    /// Builds a complete tar archive from this file system tree.
    fn build_tar(&self) -> io::Result<Writer>;
}

impl<Writer, FilePath, FileContent> BuildTar<Writer> for FileSystemTree<FilePath, FileContent>
where
    Writer: Default + Write,
    FilePath: Ord + Deref + ArchiveRoot,
    FilePath::Target:
        AsRef<Path> + JoinPath + ToOwned<Owned = <FilePath::Target as JoinPath>::OwnedPath>,
    FileContent: AsRef<[u8]>,
{
    fn build_tar(&self) -> io::Result<Writer> {
        let mut builder = tar::Builder::new(Writer::default());
        builder.append_tree(self, &FilePath::archive_root())?;
        builder.into_inner()
    }
}

impl<Writer, FilePath, FileContent> BuildTar<Writer>
    for MergeableFileSystemTree<FilePath, FileContent>
where
    Writer: Default + Write,
    FilePath: Ord + Deref + ArchiveRoot,
    FilePath::Target:
        AsRef<Path> + JoinPath + ToOwned<Owned = <FilePath::Target as JoinPath>::OwnedPath>,
    FileContent: AsRef<[u8]>,
{
    fn build_tar(&self) -> io::Result<Writer> {
        AsRef::<FileSystemTree<FilePath, FileContent>>::as_ref(self).build_tar()
    }
}

pub use tar::*;
