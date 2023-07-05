use crypto_tools::english::is_english;
use crypto_tools::xor;
use hex_literal::hex;

fn main() {
    let chipertext = hex!("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let keys = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    // Cooking MC's like a pound of bacon

    let (key, p_value, text) = keys
        .iter()
        .map(|key| {
            let text = xor::decode(&chipertext, *key);
            let p_value = is_english(&text);
            (key, p_value, text)
        })
        .max_by(|(_, p_value_a, _), (_, p_value_b, _)| p_value_a.total_cmp(p_value_b))
        .unwrap();

    let text = String::from_utf8_lossy(&text);
    // 88:Cooking MC's like a pound of bacon p: 0.8385329763799043
    println!("{key}:{text} p: {p_value}");
}
