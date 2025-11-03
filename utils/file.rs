use core::fmt::Debug;
use pipe_trait::Pipe;
use std::{fs::read_to_string, path::Path};

pub fn read_text_file(path: impl AsRef<Path> + Debug) -> String {
    path.as_ref()
        .pipe(read_to_string)
        .unwrap_or_else(|error| panic!("Failed to read {path:?}: {error}"))
}
