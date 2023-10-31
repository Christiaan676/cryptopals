use aes::{
    cipher::{
        generic_array::GenericArray, BlockDecrypt, BlockEncrypt, BlockSizeUser, Key, KeyInit,
    },
    Aes128, Aes192, Aes256,
};

pub type Aes128ECB = Ecb<Aes128>;
pub type Aes192ECB = Ecb<Aes192>;
pub type Aes256ECB = Ecb<Aes256>;

pub struct Ecb<T> {
    alg: T,
}

impl<T: KeyInit> Ecb<T> {
    pub fn new(key: &Key<T>) -> Ecb<T> {
        Ecb { alg: T::new(key) }
    }
}

impl<T: BlockSizeUser + BlockDecrypt> Ecb<T> {
    // Naive implementation only works for inputs that are N * blocksize
    pub fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        let mut output = vec![0; ciphertext.len()];

        for (in_block, out_block) in ciphertext
            .chunks_exact(T::block_size())
            .zip(output.chunks_exact_mut(T::block_size()))
        {
            let in_block = GenericArray::from_slice(in_block);
            let out_block = GenericArray::from_mut_slice(out_block);

            self.alg.decrypt_block_b2b(in_block, out_block);
        }
        output
    }
}

impl<T: BlockEncrypt + BlockSizeUser> Ecb<T> {
    // Naive implementation only works for inputs that are N * blocksize
    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let mut output = vec![0; plaintext.len()];

        for (in_block, out_block) in plaintext
            .chunks_exact(T::block_size())
            .zip(output.chunks_exact_mut(T::block_size()))
        {
            let in_block = GenericArray::from_slice(in_block);
            let out_block = GenericArray::from_mut_slice(out_block);

            self.alg.encrypt_block_b2b(in_block, out_block);
        }
        output
    }
}
