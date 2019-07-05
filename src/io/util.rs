use encoding_rs::KOI8_R;
use std::convert::TryInto;
use std::path::PathBuf;

pub trait ReadFromByteArray<T> {
    fn read_from(arr: &[u8]) -> T;
}

impl ReadFromByteArray<i16> for i16 {
    fn read_from(arr: &[u8]) -> i16 {
        read_i16(arr, 0)
    }
}

pub fn read_i16(slice: &[u8], start_from: usize) -> i16 {
    return i16::from_le_bytes(slice[start_from..start_from+2].try_into().unwrap());
}

pub fn read_i32(slice: &[u8], start_from: usize) -> i32 {
    return i32::from_le_bytes(slice[start_from..start_from+4].try_into().unwrap());
}

//pub fn read_u32(slice: &[u8], start_from: usize) -> u32 {
//    return u32::from_le_bytes(slice[start_from..start_from+4].try_into().unwrap());
//}
//
pub fn read_usize_word(slice: &[u8], start_from: usize) -> usize {
    return usize::from(
        u16::from_le_bytes(slice[start_from..start_from+2].try_into().unwrap())
    );
}

pub fn read_koi8r_str(slice: &[u8], start_from: usize, length: usize) -> String {
    let (cow_str, _, _) = KOI8_R.decode(&slice[start_from..start_from+length]);
    cow_str.as_ref().trim_matches(|c: char| c.is_whitespace() || c == '\u{0}').to_owned()
}

pub fn read_array<T: ReadFromByteArray<T>>(slice: &[u8], start_from: usize, length: usize) -> Vec<T> {
    (0..length).map(|idx|{
        T::read_from(&slice[start_from..])
    }).collect()
}

pub fn read_record<T>(byte_slice: &[u8], record_size: usize, record_number: usize, reader: impl Fn(&[u8]) -> T) -> T {
    let record = &byte_slice[record_number*record_size..(record_number+1)*record_size];
    reader(record)
}

pub fn path_in_test_directory(local_path: &str) -> String {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    d.push("test_files");
    d.push(local_path);

    String::from(d.to_str().unwrap())
}