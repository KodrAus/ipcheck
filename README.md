# IP Check

Created for https://github.com/rust-lang/rust/pull/76098

This is a little utility program for checking the behavior of various language's IP address implementations.
The goal is to make sure the Rust programs are either the same or deliberately different to other languages.

## Implementations

These live under the `impls` directory.

 - Rust (`impls/rust`)
 - .NET (`impls/dotnet`)
 - Python (`impls/python`)
 - Go (`impls/go`)
 - Java (`impls/java`)

## Running

If you happen to have all of those other languages available on your machine you can `cargo run --manifest-path host/Cargo.toml`.
