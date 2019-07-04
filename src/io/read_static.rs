use std::fs::read;
use encoding_rs::KOI8_R;
use std::mem::transmute;
use std::mem;
use std::convert::TryInto;

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

#[derive(Debug, PartialEq)]
pub struct Coords {
    x: i16,
    y: i16
}

#[derive(Debug)]
pub struct Beamspec {
    name: String,
    mc: i16,
    tri: i16,
    mol: i16,
    dur: i16,
    mass: i16,
    tech: i16,
    kill: i16,
    damage: i16
}

#[derive(Debug)]
pub struct Engspec {
    name: String,
    mc: i16,
    tri: i16,
    dur: i16,
    mol: i16,
    tech: i16,
    fuel_consumption: Vec<i32>
}

#[derive(Debug)]
pub struct Torpspec {
    name: String,
    torp_cost_mc: i16,
    mc: i16,
    tri: i16,
    dur: i16,
    mol: i16,
    mass: i16,
    tech: i16,
    kill: i16,
    damage: i16
}

#[derive(Debug)]
pub struct Hullspec {
    name: String,
    pic_number: i16,
    tri: i16,
    dur: i16,
    mol: i16,
    max_fuel: i16,
    max_crew: i16,
    engines_number: i16,
    mass: i16,
    tech: i16,
    cargo: i16,
    fighter_bays: i16,
    max_launchers: i16,
    max_beams: i16,
    mc: i16
}

const NUMBER_OR_RACES: usize = 11;
const NUMBER_OF_PLANETS: usize = 500;
const NUMBER_OF_BEAMS: usize = 10;
const NUMBER_OF_ENGINES: usize = 9;
const NUMBER_OF_WARPS: usize = 9;
const NUMBER_OF_TLAUNCHERS: usize = 10;
const NUMBER_OF_HULLS: usize = 105;

fn read_int16(slice: [u8; 2]) -> i16 {
    return i16::from_le_bytes(slice);
}

fn read_koi8r_str(slice: &[u8]) -> String {
    let (cow_str, _, _) = KOI8_R.decode(slice);
    cow_str.as_ref().trim_matches(|c: char| c.is_whitespace() || c == '\u{0}').to_owned()
}

fn read_int32(slice: [u8; 4]) -> i32 {
    return i32::from_le_bytes(slice);
}

pub fn read_race_nm(path: &str) -> Vec<RaceName> {
    let content = read(path).unwrap();

    const LONG_NAME_LENGTH: usize = 30;
    const SHORT_NAME_LENGTH: usize = 20;
    const ADJECTIVE_LENGTH: usize = 12;

    let long_names = &content[0..LONG_NAME_LENGTH*NUMBER_OR_RACES];
    let short_names = &content[LONG_NAME_LENGTH*NUMBER_OR_RACES..LONG_NAME_LENGTH*NUMBER_OR_RACES + SHORT_NAME_LENGTH*NUMBER_OR_RACES];
    let adjectives = &content[LONG_NAME_LENGTH*NUMBER_OR_RACES + SHORT_NAME_LENGTH*NUMBER_OR_RACES..];

    let race_names = (0..NUMBER_OR_RACES).map(|idx| {
        let long_name_index = idx*LONG_NAME_LENGTH;
        let short_name_index = idx*SHORT_NAME_LENGTH;
        let adjective_index = idx*ADJECTIVE_LENGTH;
        let long_name = read_koi8r_str(&long_names[long_name_index..(long_name_index+LONG_NAME_LENGTH)]);
        let short_name = read_koi8r_str(&short_names[short_name_index..(short_name_index+SHORT_NAME_LENGTH)]);
        let adjective = read_koi8r_str(&adjectives[adjective_index..(adjective_index+ADJECTIVE_LENGTH)]);
        RaceName {
            long: long_name.trim().to_owned(),
            short: short_name.trim().to_owned(),
            adjective: adjective.trim().to_owned()
        }
    }).collect();

    race_names
}

pub fn read_planet_nm(path: &str) -> Vec<PlanetName> {
    let full_str = read(path).unwrap();

    const PLANET_NAME_LENGTH: usize = 20;

    let planet_names = (0..NUMBER_OF_PLANETS).map(|idx| {
       let name = read_koi8r_str(&full_str[idx*PLANET_NAME_LENGTH..(idx+1)*PLANET_NAME_LENGTH]);
        PlanetName {
            text: name
        }
    }).collect();

    planet_names
}

pub fn read_xyplan_dat(path: &str) -> Vec<Coords> {
    let content = read(path).unwrap();
    const RECORD_SIZE: usize = 6;

    let coords = (0..NUMBER_OF_PLANETS).map(|idx| {
        let record = &content[idx*RECORD_SIZE..(idx+1)*RECORD_SIZE];
        let x = read_int16(record[0..2].try_into().unwrap());
        let y = read_int16(record[2..4].try_into().unwrap());
        Coords{
            x: x,
            y: y
        }
    }).collect();

    coords
}

pub fn read_beamspec_dat(path: &str) -> Vec<Beamspec> {
    let content = read(path).unwrap();
    const RECORD_SIZE: usize = 36;

    let specs = (0..NUMBER_OF_BEAMS).map(|idx| {
        let record = &content[idx*RECORD_SIZE..(idx+1)*RECORD_SIZE];
        Beamspec {
            name: read_koi8r_str(&record[0..20]),
            mc: read_int16(record[20..22].try_into().unwrap()),
            tri: read_int16(record[22..24].try_into().unwrap()),
            dur: read_int16(record[24..26].try_into().unwrap()),
            mol: read_int16(record[26..28].try_into().unwrap()),
            mass: read_int16(record[28..30].try_into().unwrap()),
            tech: read_int16(record[30..32].try_into().unwrap()),
            kill: read_int16(record[32..34].try_into().unwrap()),
            damage: read_int16(record[34..36].try_into().unwrap()),
        }
    }).collect();

    specs
}

pub fn read_engspec_dat(path: &str) -> Vec<Engspec> {
    let content = read(path).unwrap();
    const RECORD_SIZE: usize = 66;

    let specs = (0..NUMBER_OF_ENGINES).map(|idx| {
        let record = &content[idx*RECORD_SIZE..(idx+1)*RECORD_SIZE];
        Engspec {
            name: read_koi8r_str(&record[0..20]),
            mc: read_int16(record[20..22].try_into().unwrap()),
            tri: read_int16(record[22..24].try_into().unwrap()),
            dur: read_int16(record[24..26].try_into().unwrap()),
            mol: read_int16(record[26..28].try_into().unwrap()),
            tech: read_int16(record[28..30].try_into().unwrap()),
            fuel_consumption: (0..NUMBER_OF_WARPS).map(|x| {
                let start_index = 30 + x*mem::size_of::<i32>();
                read_int32(record[start_index..start_index + mem::size_of::<i32>()].try_into().unwrap())
            }).collect()
        }
    }).collect();

    specs
}

pub fn read_torpspec_dat(path: &str) -> Vec<Torpspec> {
    let content = read(path).unwrap();
    const RECORD_SIZE: usize = 38;

    let specs = (0..NUMBER_OF_TLAUNCHERS).map(|idx| {
        let record = &content[idx*RECORD_SIZE..(idx+1)*RECORD_SIZE];
        Torpspec {
            name: read_koi8r_str(&record[0..20]),
            torp_cost_mc: read_int16(record[20..22].try_into().unwrap()),
            mc: read_int16(record[22..24].try_into().unwrap()),
            tri: read_int16(record[24..26].try_into().unwrap()),
            dur: read_int16(record[26..28].try_into().unwrap()),
            mol: read_int16(record[28..30].try_into().unwrap()),
            mass: read_int16(record[30..32].try_into().unwrap()),
            tech: read_int16(record[32..34].try_into().unwrap()),
            kill: read_int16(record[34..36].try_into().unwrap()),
            damage: read_int16(record[36..38].try_into().unwrap()),
        }
    }).collect();

    specs
}

pub fn read_hullspec_dat(path: &str) -> Vec<Hullspec> {
    let content = read(path).unwrap();
    const RECORD_SIZE: usize = 60;

    let specs = (0..NUMBER_OF_HULLS).map(|idx| {
        let record = &content[idx*RECORD_SIZE..(idx+1)*RECORD_SIZE];
        Hullspec {
            name: read_koi8r_str(&record[0..30]),
            pic_number: read_int16(record[30..32].try_into().unwrap()),
            tri: read_int16(record[34..36].try_into().unwrap()),
            dur: read_int16(record[36..38].try_into().unwrap()),
            mol: read_int16(record[38..40].try_into().unwrap()),
            max_fuel: read_int16(record[40..42].try_into().unwrap()),
            max_crew: read_int16(record[42..44].try_into().unwrap()),
            engines_number: read_int16(record[44..46].try_into().unwrap()),
            mass: read_int16(record[46..48].try_into().unwrap()),
            tech: read_int16(record[48..50].try_into().unwrap()),
            cargo: read_int16(record[50..52].try_into().unwrap()),
            fighter_bays: read_int16(record[52..54].try_into().unwrap()),
            max_launchers: read_int16(record[54..56].try_into().unwrap()),
            max_beams: read_int16(record[56..58].try_into().unwrap()),
            mc: read_int16(record[58..60].try_into().unwrap()),
        }
    }).collect();

    specs
}

#[cfg(test)]
#[path = "./read_static_test.rs"]
mod read_static_test;