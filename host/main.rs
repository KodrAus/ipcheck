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
    let mut impls : Vec<String> = std::fs::read("../artifacts/.impls")
        .expect("missing .impls file")
        .lines()
        .map(|line| {
            let lang = line.expect("invalid .impls file");
            println!("Found {} implementation.", lang);
            lang
        })
        .collect();

    if impls.is_empty() || impls.remove(0) != "Rust" {
        println!("No Rust implementation found.");
        return;
    }

    if impls.is_empty() {
        println!("No implementations to compare against found.");
        return;
    }

    println!();

    for line in Cursor::new(fs::read("input.txt").expect("missing input.txt file")).lines() {
        let addr = line.expect("invalid input");
        print!("{}", addr);

        let rust = rust(&addr);

        let outputs = impls
            .iter()
            .map(|lang| match lang.as_str() {
                ".NET" => (lang, dotnet(&addr)),
                _ => panic!("unrecognized lang")
            });

        let diffs = outputs.clone()
            .filter(|(_, output)| output != &rust)
            .map(|(lang, _)| lang)
            .collect::<Vec<_>>();

        print!(", different: {:?}", diffs);
        print!(", Rust: {}", rust);
        for (lang, output) in outputs {
            print!(", {}: {}", lang, output);
        }

        println!();
    }
}

fn rust(addr: &str) -> Value {
    let r = generic(addr, "../artifacts/rust/ipcheck");

    serde_json::from_str(&r).expect("failed to parse output")
}

fn dotnet(addr: &str) -> Value {
    let r = generic(addr, "../artifacts/dotnet/IPCheck");

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
