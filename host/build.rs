use std::process::Command;

fn main() {
    build("cargo", "build", "../impls/ip-rust");
    build("dotnet", "build", "../impls/IPNet");
}

fn build(cmd: &str, arg: &str, wd: &str) {
    let status = Command::new(cmd).arg(arg).current_dir(wd).spawn().expect("failed to invoke executable").wait().expect("failed to wait on executable");
    assert!(status.success(), "build command failed");
}
