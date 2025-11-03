#[cfg(feature = "tar")]
mod sealed {
    use build_fs_tree::{FileSystemTree, MergeableFileSystemTree};

    pub trait ArchiveBuilder {}
    pub trait FsTree {}

    impl<Path: Ord, FileContent> FsTree for FileSystemTree<Path, FileContent> {}
    impl<Path: Ord, FileContent> FsTree for MergeableFileSystemTree<Path, FileContent> {}
}

pub mod misc;

#[cfg(feature = "tar")]
pub mod tar;

#[cfg(feature = "tar")]
pub use tar::{AppendToTar, AppendTree, BuildTar};

pub use build_fs_tree as tree;

#[cfg(feature = "tar")]
#[doc = include_str!("../README.md")]
mod _check_readme_code_examples {}
