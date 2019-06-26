mod io;

fn main() {

    let race_names = io::read_nm::read_race_nm("/Users/mgirkin/proj/rust/rust_planets/test_files/pleiades10/race.nm");

    let planet_names = io::read_nm::read_planet_nm("/Users/mgirkin/proj/rust/rust_planets/test_files/pleiades10/planet.nm");

    println!("race.nm: {:?}", race_names);
    println!("planet.nm: {:?}", planet_names);
}
