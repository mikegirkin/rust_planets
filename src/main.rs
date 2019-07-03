use crate::io::read_static::read_xyplan_dat;

mod io;

fn main() {

    let test_path = "/Users/mgirkin/proj/rust/rust_planets/test_files/pleiades10/".to_string();

    let race_names = io::read_static::read_race_nm(&format!("{}{}", test_path, "race.nm"));

    let planet_names = io::read_static::read_planet_nm(&format!("{}{}", test_path, "planet.nm"));

    let planet_coords = io::read_static::read_xyplan_dat(&format!("{}{}", test_path, "xyplan.dat"));

    let beam_specs = io::read_static::read_beamspec_dat(&format!("{}{}", test_path, "beamspec.dat"));

    let engine_specs = io::read_static::read_engspec_dat(&format!("{}{}", test_path, "engspec.dat"));

    let torp_specs = io::read_static::read_torpspec_dat(&format!("{}{}", test_path, "torpspec.dat"));

    println!("race.nm: {:?}", race_names);
    println!("planet.nm: {:?}", planet_names);
    println!("xyplan.dat: {:?}", planet_coords);
    println!("beamspec.dat: {:?}", beam_specs);
    println!("engspec.dat: {:?}", engine_specs);
    println!("torpspec.dat: {:?}", torp_specs);
}
