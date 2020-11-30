pub fn build() -> std::io::Result<()> {
    let output = std::process::Command::new("cargo")
        .args(&[
            "build",
            "--release",
            "-Z", "unstable-options",
            "--out-dir", "../../artifacts/rust",
        ])
        .current_dir("../impls/rust")
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", String::from_utf8_lossy(&output.stderr))))
    }
}