use std::io::{Cursor, BufRead};
use serde_json::{Value, Map};

fn main() {
    let mut impls : Vec<(String, String)> = std::fs::read("../artifacts/.impls")
        .expect("missing .impls file")
        .lines()
        .map(|line| line.expect("invalid .impls file"))
        .map(|line| {
            let mut line = line.split(": ");
            let lang = line.next().expect("missing impl language id");
            let artifact = line.next().expect("missing impl artifact path");

            println!("Found {} implementation.", lang);

            (lang.into(), artifact.into())
        })
        .collect();

    if impls.is_empty() || impls.get(0).map_or(false, |(lang, _)| lang != "Rust") {
        println!("No Rust implementation found.");
        return;
    }

    let (_, rust) = &impls.remove(0);

    if impls.is_empty() {
        println!("No implementations to compare against found.");
        return;
    }

    println!();

    for line in Cursor::new(std::fs::read("input.txt").expect("missing input.txt file")).lines() {
        let addr = &line.expect("invalid input");
        print!("{}", addr);

        let rust = invoke_impl("Rust", rust, addr);

        let outputs = impls
            .iter()
            .map(|(lang, artifact)| (lang, invoke_impl(lang, artifact, addr)));

        let diffs = outputs.clone()
            .filter(|(_, output)| output != &rust)
            .map(|(lang, _)| lang)
            .collect::<Vec<_>>();

        print!(", different: {:?}", diffs);
        print!(", Rust: {:?}", rust);
        for (lang, output) in outputs {
            print!(", {}: {:?}", lang, output);
        }

        println!();
    }
}

fn invoke_impl(lang: &str, artifact: &str, addr: &str) -> Map<String, Value> {
    let output = std::process::Command::new(artifact).arg(addr).output().expect(&format!("failed to invoke {} artifact", lang));

    if output.status.success() {
        let out = String::from_utf8(output.stdout).expect(&format!("failed to parse {} artifact output", lang));
        serde_json::from_str(&out).expect(&format!("failed to parse {} artifact output {:?}", lang, out))
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        panic!("{} impl error: {:?}", lang, err)
    }
}