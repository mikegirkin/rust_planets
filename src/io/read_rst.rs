use std::fs::read;
use crate::io::util::{read_usize_word, read_i16, read_record, read_koi8r_str, read_i32};
use std::convert::TryInto;

#[derive(Debug)]
pub struct FCode(String);

#[derive(Debug)]
pub struct RawRstData {
    pub general_info: RstGeneralInfo,
    pub ships: Vec<RstShipData>,
    pub visual_contacts: Vec<RstVisualContactsData>,
    pub planets: Vec<RstPlanetData>,
    pub bases: Vec<RstBaseData>,
    pub messages: Vec<RstMessage>,
    pub ship_coords: Vec<RstShipCoord>,
    pub vcrs: Vec<RstVcr>
}

#[derive(Debug)]
pub struct Minerals {
    pub neu: i32,
    pub tri: i32,
    pub dur: i32,
    pub mol: i32,
}

#[derive(Debug)]
pub struct TransferOrder {
    pub minerals: Minerals,
    pub colonists: i16,
    pub supplies: i16,
    pub target_id: i16
}

#[derive(Debug)]
pub struct RstShipData {
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
pub struct RstVisualContactsData {

}

#[derive(Debug)]
pub struct RstPlanetData {
    pub player_id: i16,
    pub planet_id: i16,
    pub fcode: FCode,
    pub number_of_mines: i16,
    pub number_of_factories: i16,
    pub number_of_defences: i16,
    pub minerals_mined: Minerals,
    pub colonist_clans: i32,
    pub supplies: i32,
    pub money: i32,
    pub minerals_ground: Minerals,
    pub minerals_density: Minerals,
    pub colonist_taxes: i16,
    pub native_taxes: i16,
    pub colonist_happines: i16,
    pub native_happines: i16,
    pub native_government: i16,
    pub native_clans: i32,
    pub native_race: i16,
    pub temperatue: i16,
    pub build_base: i16
}

#[derive(Debug)]
pub struct RstBaseData {

}

#[derive(Debug)]
pub struct RstMessage {

}

#[derive(Debug)]
pub struct RstShipCoord {

}

#[derive(Debug)]
pub struct RstGeneralInfo {

}

#[derive(Debug)]
pub struct RstVcr {

}

pub fn read_rst(path: &str) -> RawRstData {
    let content = read(path).unwrap();
    let ship_pointer = read_usize_word(&content, 0) - 1;
    let visual_contact_pointer = read_usize_word(&content, 4) - 1;
    let planet_pointer = read_usize_word(&content, 8) - 1;
    let base_pointer = read_usize_word(&content, 12) - 1;
    let message_pointer = read_usize_word(&content, 16) - 1;
    let ship_coord_pointer = read_usize_word(&content, 20) - 1;
    let geninfo_pointer = read_usize_word(&content, 24) - 1;
    let vcr_pointer = read_usize_word(&content, 28) - 1;

    RawRstData {
        ships: read_ships(&content, ship_pointer),
        visual_contacts: read_visual_contacts(&content, visual_contact_pointer),
        planets: read_planets(&content, planet_pointer),
        bases: read_bases(&content, base_pointer),
        messages: read_messages(&content, message_pointer),
        ship_coords: read_ship_coords(&content, ship_coord_pointer),
        general_info: read_geninfo(&content, geninfo_pointer),
        vcrs: read_vcrs(&content, vcr_pointer)
    }
}

fn read_ships(rst_content: &[u8], pointer: usize) -> Vec<RstShipData> {
    let ship_data = &rst_content[pointer..];
    let ships_number = read_usize_word(ship_data, 0);
    const RECORD_SIZE: usize = 107;

    (0..ships_number).map(|idx| {
        read_record(&ship_data[2..], RECORD_SIZE, idx, |slice| {
            RstShipData {
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
        minerals: read_minerals_i16(content, 0),
        colonists: read_i16(slice, 8),
        supplies: read_i16(slice, 10),
        target_id: read_i16(slice, 12),
    }
}

fn read_minerals_i16(content: &[u8], start_from: usize) -> Minerals {
    let slice = &content[start_from..];

    Minerals {
        neu: i32::from(read_i16(slice, 0)),
        tri: i32::from(read_i16(slice, 2)),
        dur: i32::from(read_i16(slice, 4)),
        mol: i32::from(read_i16(slice, 6)),
    }
}

fn read_minerals_i32(content: &[u8], start_from: usize) -> Minerals {
    let slice = &content[start_from..];

    Minerals {
        neu: read_i32(slice, 0),
        tri: read_i32(slice, 4),
        dur: read_i32(slice, 8),
        mol: read_i32(slice, 12),
    }
}


fn read_visual_contacts(rst_file: &[u8], start_from: usize) -> Vec<RstVisualContactsData> {
    Vec::new()
}

fn read_planets(rst_file: &[u8], start_from: usize) -> Vec<RstPlanetData> {
    const RECORD_SIZE: usize = 85;
    let planet_section = &rst_file[start_from..];
    let number_of_records = read_usize_word(planet_section, 0);

    (0..number_of_records).map(|idx| {
        read_record(&planet_section[2..], RECORD_SIZE, idx, |slice| {
            RstPlanetData {
                player_id: read_i16(slice, 0),
                planet_id: read_i16(slice, 2),
                fcode: FCode(read_koi8r_str(slice, 4, 3)),
                number_of_mines: read_i16(slice, 7),
                number_of_factories: read_i16(slice, 9),
                number_of_defences: read_i16(slice, 11),
                minerals_mined: read_minerals_i32(slice, 13),
                colonist_clans: read_i32(slice, 29),
                supplies: read_i32(slice, 33),
                money: read_i32(slice, 37),
                minerals_ground: read_minerals_i32(slice, 41),
                minerals_density: read_minerals_i16(slice, 57),
                colonist_taxes: read_i16(slice, 65),
                native_taxes: read_i16(slice, 67),
                colonist_happines: read_i16(slice, 69),
                native_happines: read_i16(slice, 71),
                native_government: read_i16(slice, 73),
                native_clans: read_i32(slice, 75),
                native_race: read_i16(slice, 79),
                temperatue: read_i16(slice, 81),
                build_base: read_i16(slice, 83)
            }
        })
    }).collect()
}

fn read_bases(rst_file: &[u8], start_from: usize) -> Vec<RstBaseData> {
    Vec::new()
}

fn read_messages(rst_file: &[u8], start_from: usize) -> Vec<RstMessage> {
    Vec::new()
}

fn read_ship_coords(rst_file: &[u8], start_from: usize) -> Vec<RstShipCoord> {
    Vec::new()
}

fn read_geninfo(rst_file: &[u8], start_from: usize) -> RstGeneralInfo {
    RstGeneralInfo {}
}

fn read_vcrs(rst_file: &[u8], start_from: usize) -> Vec<RstVcr> {
    Vec::new()
}


#[cfg(test)]
#[path = "./read_rst_test.rs"]
mod read_rst_test;