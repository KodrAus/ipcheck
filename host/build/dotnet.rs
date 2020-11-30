pub fn build() -> std::io::Result<()> {
    let output = std::process::Command::new("dotnet")
        .args(&[
            "publish",
            "-c", "Release",
            "-r", { cfg_if::cfg_if! {
                if #[cfg(all(target_os = "macos", target_arch = "x86_64"))] {
                    "osx-x64"
                }
                else if #[cfg(all(windows, target_arch = "x86_64"))] {
                    "win-x64"
                }
                else {
                    "linux-x64"
                }
            }},
            "--self-contained", "true",
            "-o", "../../artifacts/dotnet",
        ])
        .current_dir("../impls/dotnet")
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", String::from_utf8_lossy(&output.stdout))))
    }
}