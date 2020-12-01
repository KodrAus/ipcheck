pub fn build() -> std::io::Result<&'static str> {
    if !std::path::Path::new("../artifacts/python").exists() {
        std::fs::create_dir("../artifacts/python").expect("failed to create Python artifacts dir");
    }

    std::fs::copy("../impls/python/ipcheck.py", "../artifacts/python/ipcheck.py")?;

    Ok("python ../artifacts/python/ipcheck.py")
}
