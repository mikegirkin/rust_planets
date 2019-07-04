use encoding_rs::KOI8_R;
use std::convert::TryInto;

pub fn read_i16(slice: &[u8]) -> i16 {
    return i16::from_le_bytes(slice[0..2].try_into().unwrap());
}

pub fn read_i32(slice: &[u8]) -> i32 {
    return i32::from_le_bytes(slice[0..4].try_into().unwrap());
}

pub fn read_u32(slice: &[u8]) -> u32 {
    return u32::from_le_bytes(slice[0..4].try_into().unwrap());
}

pub fn read_usize_word(slice: &[u8]) -> usize {
    return usize::from(
        u16::from_le_bytes(slice[0..2].try_into().unwrap())
    );
}

pub fn read_koi8r_str(slice: &[u8]) -> String {
    let (cow_str, _, _) = KOI8_R.decode(slice);
    cow_str.as_ref().trim_matches(|c: char| c.is_whitespace() || c == '\u{0}').to_owned()
}

pub fn read_record<T>(byte_slice: &[u8], record_size: usize, record_number: usize, reader: impl Fn(&[u8]) -> T) -> T {
    let record = &byte_slice[record_number*record_size..(record_number+1)*record_size];
    reader(record)
}

