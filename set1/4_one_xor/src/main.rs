use crypto_tools::english::is_english;
use crypto_tools::xor;
use hex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let keys = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let (key, p_value, text) = reader
        .lines()
        .flat_map(|line| {
            let chyphertext = hex::decode(line.unwrap()).unwrap();
            keys.iter().map(move |key| {
                let text = xor::decode(&chyphertext, *key);
                let p_value = is_english(&text);
                (key, p_value, text)
            })
        })
        .max_by(|(_, p_value_a, _), (_, p_value_b, _)| p_value_a.total_cmp(p_value_b))
        .unwrap();
    let text = String::from_utf8_lossy(&text);
    //53:Now that the party is jumping\n p: 0.948168417737217
    println!("{key}:{text} p: {p_value}");
}
