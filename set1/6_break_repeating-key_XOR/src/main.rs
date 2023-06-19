use base64::{engine::general_purpose, Engine as _};
use crypto_tools::{hack::hack_single_xor, xor};
use std::fs;

fn main() {
    let mut file = fs::read_to_string("input.txt").unwrap();
    file.retain(|c| c.is_ascii_graphic());
    let cyphertext = general_purpose::STANDARD.decode(file).unwrap();

    let mut key_options = (2..=40)
        .into_iter()
        .map(|key_size| {
            let mut key_size_iter = cyphertext.chunks(key_size);

            let first_block = key_size_iter.next().unwrap();
            let second_block = key_size_iter.next().unwrap();
            let hamming_distance =
                hamming::distance(first_block, second_block) as f64 / key_size as f64;

            (key_size, hamming_distance)
        })
        .collect::<Vec<_>>();
    key_options.sort_by(|(_, a), (_, b)| a.total_cmp(b));
    key_options.truncate(5);

    for (key_size, hamming) in key_options {
        let key = (0..key_size)
            .into_iter()
            .map(|n| {
                let cyphertext = cyphertext
                    .chunks(key_size)
                    .filter_map(|chunk| chunk.get(n).cloned())
                    .collect::<Vec<_>>();
                hack_single_xor(&cyphertext)
            })
            .collect::<Vec<_>>();
        println!("Key: {key:#?}");

        let message = xor::decode_key(&cyphertext, &key);
        let message = String::from_utf8_lossy(&message);
        println!("Key: {key:#?}, msg: {message}");
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_hamming() {
        let hamming = hamming::distance(b"this is a test", b"wokka wokka!!!");
        assert_eq!(hamming, 37);
    }
}
