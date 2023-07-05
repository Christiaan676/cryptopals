use crate::{english::is_english, xor};

//const keys: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn hack_single_xor(chyphertext: &[u8]) -> (u8, f64, Vec<u8>) {
    let (key, p_value, text) = (0..=255)
        .into_iter()
        .map(|key| {
            let text = xor::decode(&chyphertext, key);
            let p_value = is_english(&text);
            (key, p_value, text)
        })
        .max_by(|(_, p_value_a, _), (_, p_value_b, _)| p_value_a.total_cmp(p_value_b))
        .unwrap();

    //let text_str = String::from_utf8_lossy(&text);
    //println!("Single key: {key:#?}, p: {p_value}, {text_str}");
    (key, p_value, text.to_vec())
}
