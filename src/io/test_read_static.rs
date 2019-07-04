
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

#[test]
fn test_read_xyplan_dat() {
    let path = path_in_test_directory("pleiades10/xyplan.dat");
    let planet_coords = read_xyplan_dat(&path);

    assert_eq!(planet_coords[0], Coords{ x: 1167, y: 1766 });
    assert_eq!(planet_coords[499], Coords{ x: 2623, y: 2579 });
}

#[test]
fn test_read_beamspec_dat() {
    let path = path_in_test_directory("pleiades10/beamspec.dat");
    let beamspec = read_beamspec_dat(&path);

    assert_eq!(beamspec[0].name, "Laser Cannon");
    assert_eq!(beamspec[0].damage, 7);

    assert_eq!(beamspec[9].name, "Multitraf Spiral");
    assert_eq!(beamspec[9].damage, 80);
}

#[test]
fn test_read_engspec_dat() {
    let path = path_in_test_directory("pleiades10/engspec.dat");
    let engspec = read_engspec_dat(&path);

    assert_eq!(engspec[0].name, "Impulse Drive");
    assert_eq!(engspec[0].fuel_consumption[8], 64800);

    assert_eq!(engspec[8].name, "Bistromatic Drive");
    assert_eq!(engspec[8].fuel_consumption[8], 6480);
}

#[test]
fn test_read_toprspec_dat() {
    let path = path_in_test_directory("pleiades10/torpspec.dat");
    let torpspec = read_torpspec_dat(&path);

    assert_eq!(torpspec[0].name, "Space Rocket");
    assert_eq!(torpspec[0].damage, 30);

    assert_eq!(torpspec[9].name, "Selphyr-Fataro-Dev.");
    assert_eq!(torpspec[9].damage, 99);
}

#[test]
fn test_read_hullspec_dat() {
    let path = path_in_test_directory("pleiades10/hullspec.dat");
    let hullspec = read_hullspec_dat(&path);

    assert_eq!(hullspec[0].name, "DAEDALUS CLASS SCOUT");
    assert_eq!(hullspec[0].mc, 100);

    assert_eq!(hullspec[NUMBER_OF_HULLS-1].name, "MERLIN CLASS ALCHEMY SHIP");
    assert_eq!(hullspec[NUMBER_OF_HULLS-1].mc, 2310);
}