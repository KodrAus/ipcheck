use std::{
    fs,
    path::Path,
    process::Command,
};

const DOTNET_RID: &'static str = {
    cfg_if::cfg_if! {
        if #[cfg(all(target_os = "macos", target_arch = "x86_64"))] {
            "osx-x64"
        }
        else if #[cfg(all(windows, target_arch = "x86_64"))] {
            "win-x64"
        }
        else {
            "linux-x64"
        }
    }
};

fn main() {
    if !Path::new("../../artifacts").exists() {
        fs::create_dir("../../artifacts").expect("failed to create artifacts dir");
    }

    build(
        "cargo",
        &[
            "build",
            "--release",
            "-Z", "unstable-options",
            "--out-dir", "../../artifacts/rust"
        ],
        "../impls/ip-rust"
    );
    build(
        "dotnet",
        &[
            "publish",
            "-c", "Release",
            "-r", DOTNET_RID,
            "--self-contained", "true",
            "-o", "../../artifacts/dotnet"
        ],
        "../impls/IPNet"
    );
}

fn build(cmd: &str, args: &[&str], wd: &str) {
    let status = Command::new(cmd).args(args).current_dir(wd).spawn().expect("failed to invoke executable").wait().expect("failed to wait on executable");
    assert!(status.success(), "build command failed");
}
