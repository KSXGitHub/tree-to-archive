use cargo_toml::Manifest;
use pipe_trait::Pipe;

#[test]
fn sync_rust_version() {
    let msrv = include_str!("../Cargo.toml")
        .pipe(Manifest::from_str)
        .unwrap()
        .package
        .unwrap()
        .rust_version
        .unwrap();
    let msrv = msrv.get().unwrap();
    let toolchain = include_str!("../rust-toolchain").trim_end();
    assert_eq!(msrv, toolchain);
}
