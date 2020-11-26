use std::{
    io::{
        Cursor,
        BufRead,
    },
    fs,
    process::Command,
};

use serde_json::{json, Value};

fn main() {
    for line in Cursor::new(fs::read("input.txt").expect("missing input.txt file")).lines() {
        let addr = line.expect("invalid input");

        let rust = rust(&addr);
        let dotnet = dotnet(&addr);

        let others = [("dotnet", &dotnet)];

        let different = others
            .iter()
            .filter(|(_, output)| **output != rust)
            .map(|(key, _)| key)
            .collect::<Vec<_>>();

        println!("{}", json!({
            "input": addr,
            "different": different,
            "rust": rust,
            "dotnet": dotnet
        }));
    }
}

fn rust(addr: &str) -> Value {
    let r = generic(addr, "../artifacts/rust/ip-rust");

    serde_json::from_str(&r).expect("failed to parse output")
}

fn dotnet(addr: &str) -> Value {
    let r = generic(addr, "../artifacts/dotnet/IPNet");

    serde_json::from_str(&r).expect("failed to parse output")
}

fn generic(addr: &str, bin: &str) -> String {
    let r = Command::new(bin)
        .arg(addr)
        .output()
        .expect("failed to invoke executable")
        .stdout;

    String::from_utf8(r).expect("invalid command output")
}
