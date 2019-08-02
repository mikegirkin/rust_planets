use std::path::{Path, PathBuf};
use crate::io::read_static::{read_xyplan_dat, read_planet_nm, PlanetName, Engspec, read_engspec_dat};
use crate::io::read_rst::{RawRstData, read_rst};

const DEFAULT_PLANET_NAME: &str = "DEFAULT_PLANET_NAME";
const XYPLAN_DAT_FILENAME: &str = "xyplan.dat";
const PLANET_NM_FILENAME: &str = "planet.nm";
const ENGSPEC_DAT_FILENAME: &str = "engspec.dat";

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Coords {
    pub x: i16,
    pub y: i16
}

#[derive(Debug, Clone)]
pub struct StaticPlanet {
    pub position: Coords,
    pub name: PlanetName
}

#[derive(Debug, Clone)]
pub struct GameRules {
    pub engines: Vec<Engspec>
}

#[derive(Debug, Clone)]
pub struct TurnData {
    pub rst_data: RawRstData
}

#[derive(Debug, Clone)]
pub struct Game {
    directory: PathBuf,
    pub planets: Vec<StaticPlanet>,
    pub rules: GameRules,
    pub turns: Vec<TurnData>
}

impl Game {
    pub fn read<P: AsRef<Path>>(directory: P) -> Game {
        let coords = read_xyplan_dat(directory.as_ref().join(XYPLAN_DAT_FILENAME));
        let planet_names = read_planet_nm(directory.as_ref().join(PLANET_NM_FILENAME));

        let static_planets = coords.iter().enumerate().map(|(index, item)| {
            StaticPlanet {
                position: item.clone(),
                name: planet_names.get(index).unwrap_or(&PlanetName(DEFAULT_PLANET_NAME.to_string())).clone()
            }
        }).collect();

        let engines = read_engspec_dat(directory.as_ref().join(ENGSPEC_DAT_FILENAME));

        let rst = read_rst(directory.as_ref().join("001").join("player2.rst"));

        Game {
            directory: directory.as_ref().to_path_buf(),
            planets: static_planets,
            rules: GameRules {
                engines: engines
            },
            turns: vec![
                TurnData {
                    rst_data: rst
                }
            ]
        }
    }
}

#[cfg(test)]
#[path = "./game_test.rs"]
mod game_test;