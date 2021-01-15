pub fn build() -> std::io::Result<&'static str> {
    if !std::path::Path::new("../artifacts/java").exists() {
        std::fs::create_dir("../artifacts/java").expect("failed to create Java artifacts dir");
    }

    std::fs::copy(
        "../impls/java/IpCheck.java",
        "../artifacts/java/IpCheck.java",
    )?;

    Ok("java ../artifacts/java/IpCheck.java")
}
