use std::path::PathBuf;
use std::{env, fs};

pub fn read_test_file(filename: &str) -> String {
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let path = dir.join("tests").join("testdata").join(filename);
    fs::read_to_string(path).expect("couldn't load test file")
}
