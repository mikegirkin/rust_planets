use std::path::PathBuf;

pub fn path_in_test_directory(local_path: &str) -> String {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    d.push("test_files");
    d.push(local_path);

    String::from(d.to_str().unwrap())
}