# test-cdylib

[![CI](https://github.com/Jarcho/test-cdylib/workflows/CI/badge.svg?branch=master&event=push)](https://github.com/Jarcho/test-cdylib/actions?query=workflow%3A%22CI%22)
[![Latest Version](https://img.shields.io/crates/v/test-cdylib.svg)](https://crates.io/crates/test-cdylib)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/test-cdylib)
[![Rustc Version 1.31+](https://img.shields.io/badge/rustc-1.31+-lightgray.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)

test-cdylib is a library for dynamically linking to cdylib projects from
test code. This allows testing for the existence of exported items.

This library is based off of dtolnay's
[TryBuild](https://crates.io/crates/trybuild).

## Testing a cdylib project

A cdylib project can be tested like this:

```rust
#[test]
fn api_test() {
    let dylib_path = test_cdylib::build_current_project();

    // Or load the shared library using any other method of your choice.
    let dylib = dlopen::symbor::Library::open(&dylib_path).unwrap();

    // Test the api as necessary.
}
```

This will build the current project, if it is not already built, and return
the path to the compiled library.

## Testing a cdylib building library

Libraries that are meant to help create cdylib interfaces can be tested in two
ways. First is to link to an example, e.g.

```rust
#[test]
fn api_gen_test() {
    let dylib_path = test_cdylib::build_example("example");

    // Or load the shared library using any other method of your choice.
    let dylib = dlopen::symbor::Library::open(&dylib_path).unwrap();

    // Test the api as necessary.
}
```

This will build the example and return the path to the compiled library.

The second way is to build a file as a cdylib, e.g.

```rust
#[test]
fn api_gen_test() {
    let dylib_path = test_cdylib::build_path("tests/cdylib/api_test.rs");

    // Or load the shared library using any other method of your choice.
    let dylib = dlopen::symbor::Library::open(&dylib_path).unwrap();

    // Test the api as necessary.
}
```

This will build the given file as a cdylib project, and return the path to
the compiled library. All dependencies and dev-dependencies are available. Note
that this will cause all dependencies to be rebuilt, which can slow down testing
significantly.

## Multiple tests with the same library

Multiple tests can link to the same library by using
[once_cell](https://crates.io/crates/once_cell) to contain the path to the
library, e.g.

```rust
use once_cell::sync::Lazy;
use std::path::PathBuf;
static LIB_PATH: Lazy<PathBuf> = Lazy::new(|| test_cdylib::build_current_project());
```

The same can be done for both `build_example` and `build_path`.

## License

Licensed under either of [Apache License](./LICENSE-APACHE), Version
2.0 or [MIT license](./LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
