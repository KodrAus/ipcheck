use std::{collections::HashSet, io::BufRead, fmt::Write};
use serde_json::{Value, Map};

fn main() {
    let (rust_artifact, impls) = &parse_impls();
    let addrs = parse_addrs();

    // Write a table header for the input and tested languages
    let mut header_buffer = String::new();
    let mut separator_buffer = String::new();

    write!(header_buffer, "| addr |").unwrap();
    write!(separator_buffer, "| - |").unwrap();

    for (lang, _) in impls {
        write!(header_buffer, " {} |", lang).unwrap();
        write!(separator_buffer, " - |").unwrap();
    }

    println!("{}", header_buffer);
    println!("{}", separator_buffer);

    // Operations that have no support in non-Rust implementations
    let mut no_lang_support = HashSet::new();

    for addr in &addrs {
        print!("| `{}` |", addr);

        let rust_output = invoke_impl("Rust", rust_artifact, addr);
        no_lang_support.extend(rust_output.keys().cloned());

        for (lang, artifact) in impls {
            let output = invoke_impl(lang, artifact, addr);

            // Make sure the output has the same shape as our Rust reference
            assert_eq!(rust_output.keys().collect::<Vec<_>>(), output.keys().collect::<Vec<_>>(), "Rust methods don't match {} methods", lang);

            let mut buffer = String::new();
            let mut diffs = 0;

            for ((rust_key, rust_value), (_, value)) in rust_output.iter().zip(output.iter()) {
                if value != "<unsupported>" {
                    no_lang_support.remove(rust_key);
                }

                if value != rust_value && value != "<unsupported>" && rust_value != "<unsupported>" {
                    if diffs > 0 {
                        write!(buffer, ", ").unwrap();
                    }

                    write!(buffer, "{} : {} (rust) ≠ {}", rust_key, rust_value, value).unwrap();

                    diffs += 1;
                }
            }

            if diffs == 0 {
                print!(" ✔️ |")
            } else {
                print!(" ❌ `{{ {} }}` |", buffer);
            }
        }

        println!();
    }

    // Write a list of operations with no support from other languages
    if no_lang_support.len() > 0 {
        println!();
        println!("Operations with only a Rust implementation:");

        for unsupported in no_lang_support {
            println!("- `{}`", unsupported);
        }
    }
}

fn parse_impls() -> (String, Vec<(String, String)>) {
    let mut impls : Vec<(String, String)> = std::fs::read("../artifacts/.impls")
        .expect("missing impls file")
        .lines()
        .map(|line| line.expect("invalid impls file"))
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
        std::process::exit(0);
    }

    let rust = impls.remove(0);

    if impls.is_empty() {
        println!("No implementations to compare against found.");
        std::process::exit(0);
    }

    println!();

    (rust.1, impls)
}

fn parse_addrs() -> Vec<String> {
    let addrs : Vec<String> = std::fs::read("input.txt")
        .expect("missing input file")
        .lines()
        .filter_map(|line| {
            let line = line.expect("invalid input file");

            if line.starts_with("#") || line.is_empty() {
                None
            } else {
                Some(line.into())
            }
        })
        .collect();

    if addrs.is_empty() {
        println!("No addresses found.");
        std::process::exit(0);
    }

    addrs
}

fn invoke_impl(lang: &str, artifact: &str, addr: &str) -> Map<String, Value> {
    let mut artifact = artifact.split(" ");
    let mut command = std::process::Command::new(artifact.next().expect(&format!("missing {} artifact", lang)));
    for arg in artifact {
        command.arg(arg);
    }

    let output = command.arg(addr).output().expect(&format!("failed to invoke {} artifact", lang));

    if output.stderr.is_empty() {
        let out = String::from_utf8(output.stdout).expect(&format!("failed to parse {} artifact output", lang));
        serde_json::from_str(&out).expect(&format!("failed to parse {} artifact output {:?}", lang, out))
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        panic!("{} impl error: {:?}", lang, err)
    }
}
