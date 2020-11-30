pub fn build() -> std::io::Result<()> {
    if !std::path::Path::new("../artifacts/rust").exists() {
        std::fs::create_dir("../artifacts/rust").expect("failed to create Rust artifacts dir");
    }

    std::fs::copy("../impls/rust/main.rs", "../artifacts/rust/main.rs")?;
    std::fs::copy("../impls/rust/Cargo.toml", "../artifacts/rust/Cargo.toml")?;

    let output = std::process::Command::new("cargo")
        .args(&[
            "build",
            "--release",
            "-Z", "unstable-options",
            "--out-dir", ".",
        ])
        .current_dir("../artifacts/rust")
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", String::from_utf8_lossy(&output.stderr))))
    }
}