# IP Check

Created for https://github.com/rust-lang/rust/pull/76098

This is a little utility program for checking the behavior of various language's IP address implementations.
The goal is to make sure the Rust programs are either the same or deliberately different to other languages.

## Implementations

These live under the `impls` directory.

- Rust (New) (`impls/rust`) with the behavior proposed in `#76098`
- Rust (Current) (`impls/rust_current`) with the current behavior on `nightly`
- .NET (`impls/dotnet`)
- Python (`impls/python`)
- Go (`impls/go`)
- Java (`impls/java`)

## Running

With the comparison languages available, you can run `cd host && cargo run` to compare them.

Each implementation is invoked by the `host` app, using the addresses parsed from the `host/input.txt` file.
