use super::util::read_i16;

#[derive(Debug, PartialEq)]
pub struct Coords {
    pub x: i16,
    pub y: i16
}

pub fn read_coords(slice: &[u8], start_from: usize) -> Coords {
    let record = &slice[start_from..];

    Coords {
        x: read_i16(record, 0),
        y: read_i16(record, 2)
    }
}