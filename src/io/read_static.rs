use std::fs::read;
use std::mem;
use std::convert::TryInto;

use super::util::*;

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

#[derive(Debug)]
pub struct HullAssignment {
    pub available_hulls: Vec<usize>
}

const NUMBER_OR_RACES: usize = 11;
const NUMBER_OF_PLANETS: usize = 500;
const NUMBER_OF_BEAMS: usize = 10;
const NUMBER_OF_ENGINES: usize = 9;
const NUMBER_OF_WARPS: usize = 9;
const NUMBER_OF_TLAUNCHERS: usize = 10;
const NUMBER_OF_HULLS: usize = 105;

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
        read_record(&content, RECORD_SIZE, idx, |slice| {
            Coords {
                x: read_i16(&slice[0..]),
                y: read_i16(&slice[2..])
            }
        })
    }).collect();

    coords
}

pub fn read_beamspec_dat(path: &str) -> Vec<Beamspec> {
    let content = read(path).unwrap();
    const RECORD_SIZE: usize = 36;

    let specs = (0..NUMBER_OF_BEAMS).map(|idx| {
        read_record(&content, RECORD_SIZE, idx, |slice| {
            Beamspec {
                name: read_koi8r_str(&slice[0..20]),
                mc: read_i16(&slice[20..]),
                tri: read_i16(&slice[22..]),
                dur: read_i16(&slice[24..]),
                mol: read_i16(&slice[26..]),
                mass: read_i16(&slice[28..]),
                tech: read_i16(&slice[30..]),
                kill: read_i16(&slice[32..]),
                damage: read_i16(&slice[34..]),
            }
        })
    }).collect();

    specs
}

pub fn read_engspec_dat(path: &str) -> Vec<Engspec> {
    let content = read(path).unwrap();
    const RECORD_SIZE: usize = 66;

    let specs = (0..NUMBER_OF_ENGINES).map(|idx| {
        read_record(&content, RECORD_SIZE, idx, |slice| {
            Engspec {
                name: read_koi8r_str(&slice[0..20]),
                mc: read_i16(&slice[20..]),
                tri: read_i16(&slice[22..]),
                dur: read_i16(&slice[24..]),
                mol: read_i16(&slice[26..]),
                tech: read_i16(&slice[28..]),
                fuel_consumption: (0..NUMBER_OF_WARPS).map(|x| {
                    let start_index = 30 + x * mem::size_of::<i32>();
                    read_i32(&slice[start_index..])
                }).collect()
            }
        })
    }).collect();

    specs
}

pub fn read_torpspec_dat(path: &str) -> Vec<Torpspec> {
    let content = read(path).unwrap();
    const RECORD_SIZE: usize = 38;

    let specs = (0..NUMBER_OF_TLAUNCHERS).map(|idx| {
        read_record(&content, RECORD_SIZE, idx, |slice| {
            Torpspec {
                name: read_koi8r_str(&slice[0..20]),
                torp_cost_mc: read_i16(&slice[20..]),
                mc: read_i16(&slice[22..]),
                tri: read_i16(&slice[24..]),
                dur: read_i16(&slice[26..]),
                mol: read_i16(&slice[28..]),
                mass: read_i16(&slice[30..]),
                tech: read_i16(&slice[32..]),
                kill: read_i16(&slice[34..]),
                damage: read_i16(&slice[36..]),
            }
        })
    }).collect();

    specs
}

pub fn read_hullspec_dat(path: &str) -> Vec<Hullspec> {
    let content = read(path).unwrap();
    const RECORD_SIZE: usize = 60;

    let specs = (0..NUMBER_OF_HULLS).map(|idx| {
        read_record(&content, RECORD_SIZE, idx, |slice| {
            Hullspec {
                name: read_koi8r_str(&slice[0..30]),
                pic_number: read_i16(&slice[30..]),
                tri: read_i16(&slice[34..]),
                dur: read_i16(&slice[36..]),
                mol: read_i16(&slice[38..]),
                max_fuel: read_i16(&slice[40..]),
                max_crew: read_i16(&slice[42..]),
                engines_number: read_i16(&slice[44..]),
                mass: read_i16(&slice[46..]),
                tech: read_i16(&slice[48..]),
                cargo: read_i16(&slice[50..]),
                fighter_bays: read_i16(&slice[52..]),
                max_launchers: read_i16(&slice[54..]),
                max_beams: read_i16(&slice[56..]),
                mc: read_i16(&slice[58..]),
            }
        })
    }).collect();

    specs
}

pub fn read_truehull_dat(path: &str) -> Vec<HullAssignment> {
    let content = read(path).unwrap();
    const RECORD_SIZE: usize = 40;

    let assignments = (0..NUMBER_OR_RACES).map(|idx| {
        read_record(&content, RECORD_SIZE, idx, |slice| {
            let avalable_hulls = (0..20).map(|x| {
                read_usize_word(&slice[x * 2..])
            }).filter(|number| *number != 0)
                .map(|number| number - 1)
                .collect::<Vec<usize>>();

            HullAssignment {
                available_hulls: avalable_hulls
            }
        })
    }).collect();

    assignments
}

#[cfg(test)]
#[path = "./test_read_static.rs"]
mod test_read_static;