#[test]
pub fn load_example() {
    let dylib = test_cdylib::build_example("test_example");
    let dylib = dlopen::symbor::Library::open(&dylib)
        .expect(&format!("failed to open library: {}", dylib.display()));
    let identity = unsafe {
        dylib
            .symbol::<extern "C" fn(i32) -> i32>("identity")
            .unwrap()
    };
    assert_eq!(identity(1), 1);
}

#[test]
#[should_panic]
pub fn missing_example() {
    test_cdylib::build_example("missing");
}
