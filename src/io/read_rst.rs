#[derive(Debug)]
pub struct RawRstData {
    pub addresses: [u32; 8]
}


pub fn read_rst(path: &str) -> RawRstData {
    unimplemented!()
}


#[cfg(test)]
#[path = "./test_read_rst.rs"]
mod test_read_rst;