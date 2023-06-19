fn main() {
    println!("Hello, world!");
}

fn to_base64(input: &[u8]) -> String {
    input.chunks(3).map(to_base64_chars).collect()
}

fn to_base64_chars(input: &[u8]) -> String {
    match input {
        i @ [_, _, _] => bits_24(i.try_into().unwrap()),
        i @ [_, _] => bits_18(i.try_into().unwrap()),
        [a] => {
            let mut result = String::new();
            result.push(to_base64_char(*a));
            result.push_str("==");
            result
        }
        _ => panic!(),
    }
}

fn bits_24(input: [u8; 3]) -> String {
    let mut word = [0; 4];
    word[1..].copy_from_slice(&input);
    let input = u32::from_be_bytes(word);
    let mut result = String::new();
    result.push(to_base64_char((input >> 18) as u8));
    result.push(to_base64_char((input >> 12) as u8));
    result.push(to_base64_char((input >> 6) as u8));
    result.push(to_base64_char(input as u8));
    result
}

fn bits_18(input: [u8; 2]) -> String {
    let input = u16::from_be_bytes(input);
    let mut result = String::new();
    result.push(to_base64_char((input >> 12) as u8));
    result.push(to_base64_char((input >> 6) as u8));
    result.push(to_base64_char(input as u8));
    result.push('=');
    result
}

const alphabet: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
fn to_base64_char(input: u8) -> char {
    let input = input & 0x3F;
    char::from(alphabet[input as usize])
}

#[cfg(test)]
mod test {
    use hex_literal::hex;

    use super::*;

    #[test]
    fn example() {
        let input = hex!("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        let base_64_result = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(to_base64(&input), base_64_result.to_string());
    }
}
