# test-cdylib

test-cdylib is a library for dynamically linking to cdylib projects from
test code. This allows testing for the existence of exported items.

This library is based off of dtolnay's
[TryBuild](https://crates.io/crates/trybuild).

## Testing a cdylib project

A cdylib project can be tested like this:

```rust
#[test]
fn api_test() {
    let dylib_path = test_cdylib::build_project();

    // Or load the shared library using any other method of your choice.
    let dylib = dlopen::symbor::Library::open(&dylib_path).unwrap();

    // Test the api as necessary.
}
```

This will build the current project, if it is not already built, and return
the path to the compiled library.

## Testing a cdylib building library

Libraries that are meant to help create cdylib interfaces can be tested like
this:

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
the compiled library. All dependencies and dev-dependencies are available.

## License

Licensed under either of [Apache License](./LICENSE-APACHE), Version
2.0 or [MIT license](./LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
