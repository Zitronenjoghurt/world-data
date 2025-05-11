use std::io::Read;
use flate2::read::ZlibDecoder;
use world_data_types::data::WorldData;

const INCLUDED_DATA: &[u8] = include_bytes!("../data.bin");
pub fn load_world_data() -> WorldData {
    let mut decompressor = ZlibDecoder::new(INCLUDED_DATA);
    let mut decompressed_data = Vec::new();
    decompressor.read_to_end(&mut decompressed_data).unwrap();
    bincode::deserialize(&decompressed_data).unwrap()
}