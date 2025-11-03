#![cfg(feature = "tar")]
use _utils::{Temp, read_text_file, tar_xvf};
use build_fs_tree::{FileSystemTree, MergeableFileSystemTree, dir, file};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use std::fs::write;
use tar::Builder;
use tree_to_archive::tar::{AppendToTar, AppendTree, BuildTar};

#[test]
fn append_to_tar() {
    let tree: FileSystemTree<&str, &str> = dir! {
        "README.md" => file!("# Example Rust project"),
        "src" => dir! {
            "main.rs" => file!("fn main() {}"),
        },
    };

    let mut builder = Builder::new(Vec::new());
    tree.append_to_tar(&mut builder, "foo/bar")
        .expect("append tree to tar");

    let temp = Temp::new("archive");
    let archive_path = temp.join("archive.tar");
    let content = builder.into_inner().expect("content");
    write(&archive_path, &content).expect("write archive to filesystem");

    tar_xvf(&archive_path, &temp);
    assert_eq!(
        temp.join("foo/bar/README.md").pipe(read_text_file),
        "# Example Rust project"
    );
    assert_eq!(
        temp.join("foo/bar/src/main.rs").pipe(read_text_file),
        "fn main() {}"
    );
}

#[test]
fn append_to_tar_mergeable() {
    let tree: MergeableFileSystemTree<&str, &str> = dir! {
        "foo/bar/README.md" => file!("# Example Rust project"),
        "foo/bar/src/main.rs" => file!("fn main() {}"),
    }
    .into();

    let mut builder = Builder::new(Vec::new());
    tree.append_to_tar(&mut builder, "")
        .expect("append tree to tar");

    let temp = Temp::new("archive");
    let archive_path = temp.join("archive.tar");
    let content = builder.into_inner().expect("content");
    write(&archive_path, &content).expect("write archive to filesystem");

    tar_xvf(&archive_path, &temp);
    assert_eq!(
        temp.join("foo/bar/README.md").pipe(read_text_file),
        "# Example Rust project"
    );
    assert_eq!(
        temp.join("foo/bar/src/main.rs").pipe(read_text_file),
        "fn main() {}"
    );
}

#[test]
fn append_tree() {
    let tree: FileSystemTree<&str, &str> = dir! {
        "README.md" => file!("# Example Rust project"),
        "src" => dir! {
            "main.rs" => file!("fn main() {}"),
        },
    };

    let mut builder = Builder::new(Vec::new());
    builder
        .append_tree(&tree, "")
        .expect("append tree to tar using AppendTree");

    let temp = Temp::new("archive");
    let archive_path = temp.join("archive.tar");
    let content = builder.into_inner().expect("content");
    write(&archive_path, &content).expect("write archive to filesystem");

    tar_xvf(&archive_path, &temp);
    assert_eq!(
        temp.join("README.md").pipe(read_text_file),
        "# Example Rust project"
    );
    assert_eq!(
        temp.join("src/main.rs").pipe(read_text_file),
        "fn main() {}"
    );
}

#[test]
fn build_tar() {
    let tree: FileSystemTree<&str, &str> = dir! {
        "README.md" => file!("# Example Rust project"),
        "src" => dir! {
            "main.rs" => file!("fn main() {}"),
        },
    };

    let tar: Vec<u8> = tree.build_tar().expect("build a tar archive from a tree");

    let temp = Temp::new("archive");
    let archive_path = temp.join("archive.tar");
    write(&archive_path, &tar).expect("write the tar archive");

    tar_xvf(&archive_path, &temp);
    assert_eq!(
        temp.join("README.md").pipe(read_text_file),
        "# Example Rust project"
    );
    assert_eq!(
        temp.join("src/main.rs").pipe(read_text_file),
        "fn main() {}"
    );
}
