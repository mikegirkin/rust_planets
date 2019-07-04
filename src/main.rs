use crate::io::read_static::{read_xyplan_dat, Hullspec};

mod io;

fn main() {

    let test_path = "/Users/mgirkin/proj/rust/rust_planets/test_files/pleiades10/".to_string();

    let race_names = io::read_static::read_race_nm(&format!("{}{}", test_path, "race.nm"));

    let planet_names = io::read_static::read_planet_nm(&format!("{}{}", test_path, "planet.nm"));

    let planet_coords = io::read_static::read_xyplan_dat(&format!("{}{}", test_path, "xyplan.dat"));

    let beam_specs = io::read_static::read_beamspec_dat(&format!("{}{}", test_path, "beamspec.dat"));

    let engine_specs = io::read_static::read_engspec_dat(&format!("{}{}", test_path, "engspec.dat"));

    let torp_specs = io::read_static::read_torpspec_dat(&format!("{}{}", test_path, "torpspec.dat"));

    let hull_specs = io::read_static::read_hullspec_dat(&format!("{}{}", test_path, "hullspec.dat"));

    let truehull_raw = io::read_static::read_truehull_dat(&format!("{}{}", test_path, "truehull.dat"));

    let hulls_races: Vec<Vec<&Hullspec>> = truehull_raw.iter().map(|assignment| {
        assignment.available_hulls.iter().map(|hull_idx| {
            &hull_specs[*hull_idx]
        }).collect()
    }).collect();

    println!("race.nm: {:?}", race_names);
    println!("planet.nm: {:?}", planet_names);
    println!("xyplan.dat: {:?}", planet_coords);
    println!("beamspec.dat: {:?}", beam_specs);
    println!("engspec.dat: {:?}", engine_specs);
    println!("torpspec.dat: {:?}", torp_specs);
    println!("hullspec.dat: {:?}", hull_specs);
    println!("Liz ships: {:?}", hulls_races[1]);
}
