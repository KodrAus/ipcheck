mod rust;
mod dotnet;

fn main() {
    if !std::path::Path::new("../../artifacts").exists() {
        std::fs::create_dir("../../artifacts").expect("failed to create artifacts dir");
    }

    rust::build();
    dotnet::build();
}