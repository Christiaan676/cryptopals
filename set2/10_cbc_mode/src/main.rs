use crypto_tools::{aes_cbc::Aes128CBC, input_file::load_file};

fn main() {
    let input = load_file("input.txt");

    let message = Aes128CBC::new(b"YELLOW SUBMARINE".into()).decrypt(&input, &[0; 16]);
    let message = String::from_utf8_lossy(&message);
    println!("{message}");
}
