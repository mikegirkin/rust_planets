
use super::*;
use std::path::PathBuf;

fn path_in_test_directory(local_path: &str) -> String {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    d.push("test_files");
    d.push(local_path);

    String::from(d.to_str().unwrap())
}

#[test]
fn test_read_race_nm() {
    let path = path_in_test_directory("pleiades10/race.nm");
    let races = read_race_nm(&path);

    assert_eq!(races[0].short, "The Feds");
    assert_eq!(races[10].adjective, "Colonial");
}

#[test]
fn test_read_planet_nm() {
    let path = path_in_test_directory("pleiades10/planet.nm");
    let planet_names = read_planet_nm(&path);

    assert_eq!(planet_names[0].text, "Ceti Alpha one");
    assert_eq!(planet_names[499].text, "Fantasia");
}