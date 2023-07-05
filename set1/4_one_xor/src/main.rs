use crypto_tools::hack::hack_single_xor;
use hex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let (key, p_value, text): (u8, f64, Vec<u8>) = reader
        .lines()
        .map(|line| {
            let chyphertext = hex::decode(line.unwrap()).unwrap();
            hack_single_xor(&chyphertext)
        })
        .max_by(|(_, p_value_a, _), (_, p_value_b, _)| p_value_a.total_cmp(p_value_b))
        .unwrap();
    let text = String::from_utf8_lossy(&text);
    //53:Now that the party is jumping\n p: 0.948168417737217
    println!("{key}:{text} p: {p_value}");
}
