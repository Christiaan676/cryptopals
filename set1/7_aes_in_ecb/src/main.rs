use std::fs;

use base64::{engine::general_purpose, Engine};
use crypto_tools::aes_ecb::Aes128ECB;

fn main() {
    let mut file = fs::read_to_string("input.txt").unwrap();
    file.retain(|c| c.is_ascii_graphic());
    let cyphertext = general_purpose::STANDARD.decode(file).unwrap();

    let key = b"YELLOW SUBMARINE";
    let ecb = Aes128ECB::new(key.into());

    let result = ecb.decrypt(&cyphertext);
    println!("msg len: {} {}", cyphertext.len(), result.len());
    let result = String::from_utf8(result).unwrap();
    println!("msg: {result}");
}
