fn main() {}

#[cfg(test)]
mod test {
    use hex_literal::hex;

    #[test]
    fn test() {
        let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = b"ICE";

        let result = hex!("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");

        let ciphtertext: Vec<u8> = input
            .iter()
            .zip(key.iter().cycle())
            .map(|(a, b)| a ^ b)
            .collect();

        assert_eq!(ciphtertext, result)
    }
}
