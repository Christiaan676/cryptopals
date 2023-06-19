fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crypto_tools::xor::xor;
    use hex_literal::hex;

    #[test]
    fn test() {
        let input_a = hex!("1c0111001f010100061a024b53535009181c");
        let input_b = hex!("686974207468652062756c6c277320657965");
        let xor = xor(&input_a, &input_b);
        assert_eq!(&xor, &hex!("746865206b696420646f6e277420706c6179"));
    }
}
