use std::{fs, path::Path};

use base64::{engine::general_purpose, Engine};

pub fn load_file<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::read_to_string(path).expect("Input file not pressent");
    file.retain(|c| c.is_ascii_graphic());
    general_purpose::STANDARD.decode(file).unwrap()
}
