pub fn build() {
    let status = std::process::Command::new("cargo")
        .args(&[
            "build",
            "--release",
            "-Z", "unstable-options",
            "--out-dir", "../../artifacts/rust",
        ])
        .current_dir("../impls/ip-rust")
        .output()
        .expect("failed to execute")
        .status;

    assert!(status.success(), "build command failed");
}