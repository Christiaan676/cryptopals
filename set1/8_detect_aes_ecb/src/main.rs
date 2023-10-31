use std::{collections::HashSet, fs};

use aes::{cipher::BlockSizeUser, Aes128};
use base64::{engine::general_purpose, Engine};

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    for line in file.lines() {
        let ciphertext = general_purpose::STANDARD.decode(line).unwrap();
        let nr_blocks = ciphertext.len() / Aes128::block_size();

        // detect blocks in the cipher text
        let block_set: HashSet<_> = ciphertext.chunks(Aes128::block_size()).collect();
        if nr_blocks > block_set.len() {
            for block in ciphertext.chunks(Aes128::block_size()) {
                println!("{block:x?}")
            }
        }
    }
}
