# tree-to-archive

Create an archive from an fs tree macro or YAML.

## Usage Example

```rust no_run
use tree_to_archive::{
    tree::{dir, file, FileSystemTree},
    tar::BuildTar,
};

let tree: FileSystemTree<&str, &str> = dir! {
    "README.md" => file!("# My Project"),
    "src" => dir! {
        "main.rs" => file!("fn main() {}"),
    },
};

let archive_data: Vec<u8> = tree.build_tar().unwrap();
```

## Documentation

See [docs.rs](https://docs.rs/tree-to-archive).

## License

[MIT](https://github.com/KSXGitHub/tree-to-archive/blob/master/LICENSE.md) © [Hoàng Văn Khải](https://github.com/KSXGitHub).
