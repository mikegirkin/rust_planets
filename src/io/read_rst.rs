use std::fs::read;
use crate::io::util::{read_usize_word, read_i16, read_record, read_koi8r_str};
use std::convert::TryInto;

#[derive(Debug)]
pub struct FCode(String);

#[derive(Debug)]
pub struct RawRstData {
    pub ships: Vec<ShipRstData>
}

#[derive(Debug)]
pub struct ShipRstData {
    pub id: i16,
    pub player_id: i16,
    pub fcode: FCode,
    pub warp: i16,
    pub x_waypoint_distance: i16,
    pub y_waypoint_distance: i16,
    pub x_position: i16,
    pub y_position: i16,
    pub engine_id: i16,
    pub hull_id: i16,
    pub beam_id: i16,
    pub number_of_beams: i16,
    pub number_of_bays: i16,
    pub launcher_id: i16,
    pub number_of_torps_fighters: i16,
    pub number_of_launchers: i16,
    pub mission_id: i16,
    pub enemy_id: i16,
    pub first_mission_argument: i16,
    pub damage: i16,
    pub crew: i16,
    pub clans: i16,
    pub name: String,
    pub neu: i16,
    pub tri: i16,
    pub dur: i16,
    pub mol: i16,
    pub supplies: i16,
    pub unload_to_planet: TransferOrder,
    pub transfer_to_ship: TransferOrder,
    pub second_mission_argument: i16,
    pub money: i16
}

#[derive(Debug)]
pub struct TransferOrder {
    pub neu: i16,
    pub tri: i16,
    pub dur: i16,
    pub mol: i16,
    pub colonists: i16,
    pub supplies: i16,
    pub target_id: i16
}

pub fn read_rst(path: &str) -> RawRstData {
    let content = read(path).unwrap();
    let ship_pointer = read_usize_word(&content, 0) - 1;

    let ships = read_ships(&content, ship_pointer);

    RawRstData {
        ships
    }
}

fn read_ships(rst_content: &[u8], pointer: usize) -> Vec<ShipRstData> {
    let ship_data = &rst_content[pointer..];
    let ships_number = read_usize_word(ship_data, 0);
    const RECORD_SIZE: usize = 107;

    (0..ships_number).map(|idx| {
        read_record(&ship_data[2..], RECORD_SIZE, idx, |slice| {
            ShipRstData {
                id: read_i16(slice, 0),
                player_id: read_i16(slice, 2),
                fcode: FCode(read_koi8r_str(&slice, 4, 3)),
                warp: read_i16(slice, 7),
                x_waypoint_distance: read_i16(slice, 9),
                y_waypoint_distance: read_i16(slice, 11),
                x_position: read_i16(slice, 13),
                y_position: read_i16(slice, 15),
                engine_id: read_i16(slice, 17),
                hull_id: read_i16(slice, 19),
                beam_id: read_i16(slice, 21),
                number_of_beams: read_i16(slice, 23),
                number_of_bays: read_i16(slice, 25),
                launcher_id: read_i16(slice, 27),
                number_of_torps_fighters: read_i16(slice, 29),
                number_of_launchers: read_i16(slice, 31),
                mission_id: read_i16(slice, 33),
                enemy_id: read_i16(slice, 35),
                first_mission_argument: read_i16(slice, 37),
                damage: read_i16(slice, 39),
                crew: read_i16(slice, 41),
                clans: read_i16(slice, 43),
                name: read_koi8r_str(slice, 45, 20),
                neu: read_i16(slice, 65),
                tri: read_i16(slice, 67),
                dur: read_i16(slice, 69),
                mol: read_i16(slice, 71),
                supplies: read_i16(slice, 73),
                unload_to_planet: read_transfer_order(slice, 75),
                transfer_to_ship: read_transfer_order(slice, 89),
                second_mission_argument: read_i16(slice, 103),
                money: read_i16(slice, 105)
            }
        })
    }).collect()
}

fn read_transfer_order(content: &[u8], start_from: usize) -> TransferOrder {
    let slice = &content[start_from..];

    TransferOrder {
        neu: read_i16(slice, 0),
        tri: read_i16(slice, 0),
        dur: read_i16(slice, 0),
        mol: read_i16(slice, 0),
        colonists: read_i16(slice, 0),
        supplies: read_i16(slice, 0),
        target_id: read_i16(slice, 0),
    }
}


#[cfg(test)]
#[path = "./read_rst_test.rs"]
mod read_rst_test;