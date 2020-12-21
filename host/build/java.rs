pub fn build() -> std::io::Result<&'static str> {
    if !std::path::Path::new("../artifacts/java").exists() {
        std::fs::create_dir("../artifacts/java").expect("failed to create Java artifacts dir");
    }

    if !std::path::Path::new("../artifacts/java/src/main/java").exists() {
        std::fs::create_dir_all("../artifacts/java/src/main/java")?
    }

    std::fs::copy("../impls/java/IpCheck.java", "../artifacts/java/src/main/java/IpCheck.java")?;
    std::fs::copy("../impls/java/pom.xml", "../artifacts/java/pom.xml")?;

    let output = std::process::Command::new("mvn")
        .arg("package")
        .current_dir("../artifacts/java")
        .output()?;

    if output.status.success() {
        Ok("java -jar ../artifacts/java/target/ipcheck-1.0-jar-with-dependencies.jar")
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", String::from_utf8_lossy(&output.stdout))))
    }
}
