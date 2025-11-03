use command_extra::CommandExtra;
use std::{
    path::Path,
    process::{Command, Stdio},
};

pub fn tar_xvf(archive_path: &Path, dest: &Path) {
    let output = Command::new("tar")
        .with_arg("xvf")
        .with_arg(archive_path)
        .with_current_dir(dest)
        .with_stdin(Stdio::null())
        .with_stdout(Stdio::piped())
        .with_stderr(Stdio::piped())
        .output()
        .expect("use tar to exact file");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success());
}
