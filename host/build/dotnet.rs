pub fn build() -> std::io::Result<()> {
    if !std::path::Path::new("../artifacts/dotnet").exists() {
        std::fs::create_dir("../artifacts/dotnet").expect("failed to create .NET artifacts dir");
    }

    std::fs::copy("../impls/dotnet/IPCheck.cs", "../artifacts/dotnet/IPCheck.cs")?;
    std::fs::copy("../impls/dotnet/IPCheck.csproj", "../artifacts/dotnet/IPCheck.csproj")?;

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
            "-o", ".",
        ])
        .current_dir("../artifacts/dotnet")
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", String::from_utf8_lossy(&output.stdout))))
    }
}