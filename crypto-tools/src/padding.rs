use std::iter::once;

fn pkcs_7(input: &[u8], block_size: u8) -> Vec<u8> {
    let mut pad_bytes = (input.len() % block_size as usize);
    if pad_bytes == 0 {
        // We always need to end with padding
        pad_bytes = block_size as usize;
    }
    let padding = once(pad_bytes as u8).cycle().take(pad_bytes);
    input.iter().cloned().chain(padding).collect()
}
