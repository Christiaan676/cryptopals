use base64::{engine::general_purpose, Engine as _};
use crypto_tools::{english::is_english, hack::hack_single_xor, xor};
use std::fs;

fn main() {
    let mut file = fs::read_to_string("input.txt").unwrap();
    file.retain(|c| c.is_ascii_graphic());
    let cyphertext = general_purpose::STANDARD.decode(file).unwrap();

    let mut key_size_options: Vec<(_, f64)> = (2..=40)
        .into_iter()
        .map(|key_size| {
            let mut key_size_iter = cyphertext.chunks(key_size).peekable();

            let mut hamming = vec![];
            for _ in 0..4 {
                let first_block = key_size_iter.next().unwrap();
                let second_block = key_size_iter.peek().unwrap();
                let hamming_distance =
                    hamming::distance(first_block, second_block) as f64 / key_size as f64;
                hamming.push(hamming_distance);
            }

            (key_size, hamming.iter().sum::<f64>() / hamming.len() as f64)
        })
        .collect::<Vec<_>>();
    key_size_options.sort_by(|(_, a), (_, b)| a.total_cmp(b));
    key_size_options.truncate(5);

    let (key, text, p) = key_size_options
        .into_iter()
        .map(|(key_size, _hamming)| {
            let key = (0..key_size)
                .into_iter()
                .map(|n| {
                    let cyphertext = cyphertext
                        .chunks(key_size)
                        .filter_map(|chunk| chunk.get(n).cloned())
                        .collect::<Vec<_>>();
                    let (key, _p_value, _text) = hack_single_xor(&cyphertext);
                    key
                })
                .collect::<Vec<_>>();
            //println!("Key: {key:#?}");

            let text = xor::decode_key(&cyphertext, &key);

            //let message: String = text.iter().map(|c| char::from(*c)).collect();
            //let message = String::from_utf8_lossy(&text);
            let p = is_english(&text);
            //println!("Key: {key:X?}, p: {p}, msg: {message}");
            (key, text, p)
        })
        .max_by(|(_, _, a), (_, _, b)| a.total_cmp(b))
        .unwrap();

    let message = String::from_utf8_lossy(&text);
    println!("Key: {key:X?}, p: {p}");
    println!("{}", message);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_hamming() {
        let hamming = hamming::distance(b"this is a test", b"wokka wokka!!!");
        assert_eq!(hamming, 37);
    }
}
