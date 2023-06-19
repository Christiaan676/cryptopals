use crate::{english::is_english, xor};

pub fn hack_single_xor(chyphertext: &[u8]) -> u8 {
    let keys = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let (key, p_value, text) = keys
        .iter()
        .map(|key| {
            let text = xor::decode(&chyphertext, *key);
            let p_value = is_english(&text);
            (key, p_value, text)
        })
        .max_by(|(_, p_value_a, _), (_, p_value_b, _)| p_value_a.total_cmp(p_value_b))
        .unwrap();

    let text = String::from_utf8_lossy(&text);
    println!("Single key: {key:#?}, p: {p_value}, {text}");
    *key
}
