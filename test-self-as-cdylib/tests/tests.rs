#[test]
pub fn load_lib() {
    let dylib = test_cdylib::build_current_project();
    let dylib = dlopen::symbor::Library::open(&dylib)
        .expect(&format!("failed to open library: {}", dylib.display()));
    let identity = unsafe {
        dylib
            .symbol::<extern "C" fn(i32) -> i32>("identity")
            .unwrap()
    };
    assert_eq!(identity(1), 1);
}
