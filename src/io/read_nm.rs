use std::fs::read;
use encoding_rs::KOI8_R;

#[derive(Debug)]
pub struct RaceName {
    short: String,
    long: String,
    adjective: String
}

#[derive(Debug)]
pub struct PlanetName {
    text: String
}

const NUMBER_OF_PLANETS: usize = 500;

fn read_koi8r_file(path: &str) -> String {
    let content = read(path).unwrap();
    let (cow_str, _, _) = KOI8_R.decode(content.as_ref());

    return cow_str.as_ref().to_owned();
}

pub fn read_race_nm(path: &str) -> Vec<RaceName> {
    let full_str = read_koi8r_file(path);

    const LONG_NAME_LENGTH: usize = 30;
    const SHORT_NAME_LENGTH: usize = 20;
    const ADJECTIVE_LENGTH: usize = 12;

    let long_names = &full_str[0..LONG_NAME_LENGTH*11];
    let short_names = &full_str[LONG_NAME_LENGTH*11..LONG_NAME_LENGTH*11 + SHORT_NAME_LENGTH*11];
    let adjectives = &full_str[LONG_NAME_LENGTH*11 + SHORT_NAME_LENGTH*11..];

    let races_names: Vec<RaceName> = (0..11).map(|idx| {
        let long_name_index = idx*LONG_NAME_LENGTH;
        let short_name_index = idx*SHORT_NAME_LENGTH;
        let adjective_index = idx*ADJECTIVE_LENGTH;
        let long_name = &long_names[long_name_index..(long_name_index+LONG_NAME_LENGTH)];
        let short_name = &short_names[short_name_index..(short_name_index+SHORT_NAME_LENGTH)];
        let adjective = &adjectives[adjective_index..(adjective_index+ADJECTIVE_LENGTH)];
        RaceName {
            long: long_name.trim().to_owned(),
            short: short_name.trim().to_owned(),
            adjective: adjective.trim().to_owned()
        }
    }).collect();

    races_names
}

pub fn read_planet_nm(path: &str) -> Vec<PlanetName> {
    let full_str = read_koi8r_file(path);

    const PLANET_NAME_LENGTH: usize = 20;

    let planet_names = (0..NUMBER_OF_PLANETS).map(|idx| {
       let name = &full_str[idx*PLANET_NAME_LENGTH..(idx+1)*PLANET_NAME_LENGTH];
        PlanetName {
            text: name.trim().to_owned()
        }
    }).collect();

    planet_names
}
