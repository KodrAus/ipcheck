mod rust;
mod dotnet;

fn main() {
    if !std::path::Path::new("../../artifacts").exists() {
        std::fs::create_dir("../../artifacts").expect("failed to create artifacts dir");
    }

    handle_err("Rust",rust::build());
    handle_err(".NET", dotnet::build());
}

fn handle_err(lang: &str, result: std::io::Result<()>) {
    if let Err(err) = result {
        panic!("Failed to build {} impl: {}", lang, err)
    }
}