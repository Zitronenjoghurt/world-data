use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use world_data_build::build_data;

fn main() {
    let data_path = PathBuf::from("../data");
    let out_dir = PathBuf::from("../");
    let dest_path = PathBuf::from(out_dir).join("data.bin");

    let data = build_data(&data_path);
    let encoded_data = bincode::serialize(&data).unwrap();

    let mut compressor = ZlibEncoder::new(Vec::new(), Compression::best());
    compressor.write_all(&encoded_data).unwrap();
    let compressed_data = compressor.finish().unwrap();

    let mut file = File::create(dest_path).unwrap();
    file.write_all(&compressed_data).unwrap();
}