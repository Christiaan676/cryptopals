use std::iter::once;

fn pkcs_7(input: &[u8], block_size: u8) -> Vec<u8> {
    let mut pad_bytes = block_size as usize - (input.len() % block_size as usize);
    if pad_bytes == 0 {
        // We always need to end with padding
        pad_bytes = block_size as usize;
    }
    let padding = once(pad_bytes as u8).cycle().take(pad_bytes);
    input.iter().cloned().chain(padding).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = pkcs_7(b"YELLOW SUBMARINE", 20);
        assert_eq!(result, b"YELLOW SUBMARINE\x04\x04\x04\x04");
    }
}
