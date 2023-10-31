use crypto_tools::aes_cbc::Aes128CBC;
use rand::prelude::*;

fn main() {
    println!("Hello, world!");

    let key: [u8; 16] = random();
}
