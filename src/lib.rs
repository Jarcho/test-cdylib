//! test-cdylib is a library for dynamically linking to cdylib projects from
//! test code. This allows testing for the existence of exported items.
//!
//! # Testing a cdylib project
//!
//! A cdylib project can be tested like this:
//!
//! ```no_run
//! #[test]
//! fn api_test() {
//!     let dylib_path = test_cdylib::build_current_project();
//!
//!     // Or load the shared library using any other method of your choice.
//!     let dylib = dlopen::symbor::Library::open(&dylib_path).unwrap();
//!
//!     // Test the api as necessary.
//! }
//! ```
//!
//! This will build the current project, if it is not already built, and return
//! the path to the compiled library.
//!
//! # Testing a cdylib building library
//!
//! Libraries that are meant to help create cdylib interfaces can be tested like
//! this:
//!
//! ```no_run
//! #[test]
//! fn api_gen_test() {
//!     let dylib_path = test_cdylib::build_path("tests/cdylib/api_test.rs");
//!
//!     // Or load the shared library using any other method of your choice.
//!     let dylib = dlopen::symbor::Library::open(&dylib_path).unwrap();
//!
//!     // Test the api as necessary.
//! }
//! ```
//!
//! This will build the given file as a cdylib project, and return the path to
//! the compiled library. All dependencies and dev-dependencies are available.

#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};

#[macro_use]
mod path;

mod cargo;
mod dependencies;
mod error;
mod features;
mod manifest;
mod run;
mod rustflags;

/// Builds the given file as a cdylib and returns the path to the compiled object.
pub fn build_file<P: AsRef<Path>>(path: P) -> PathBuf {
    run::run(path.as_ref()).unwrap()
}

/// Builds the current project as a cdylib and returns the path to the compiled object.
pub fn build_current_project() -> PathBuf {
    cargo::build_self_cdylib().unwrap()
}
