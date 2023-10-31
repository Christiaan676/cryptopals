use aes::{
    cipher::{
        generic_array::GenericArray, BlockDecrypt, BlockEncrypt, BlockSizeUser, Key, KeyInit,
    },
    Aes128, Aes192, Aes256,
};

pub type Aes128CBC = Cbc<Aes128>;
pub type Aes192CBC = Cbc<Aes192>;
pub type Aes256CBC = Cbc<Aes256>;

pub struct Cbc<T> {
    alg: T,
}

impl<T: KeyInit> Cbc<T> {
    pub fn new(key: &Key<T>) -> Cbc<T> {
        Self { alg: T::new(key) }
    }
}

impl<T: BlockSizeUser + BlockDecrypt> Cbc<T> {
    // Naive implementation only works for inputs that are N * blocksize
    pub fn decrypt(&self, ciphertext: &[u8], iv: &[u8]) -> Vec<u8> {
        let mut output = vec![0; ciphertext.len()];

        let mut buffer = GenericArray::<u8, T::BlockSize>::clone_from_slice(iv);

        for (in_block, out_block) in ciphertext
            .chunks_exact(T::block_size())
            .zip(output.chunks_exact_mut(T::block_size()))
        {
            let in_block = GenericArray::<u8, T::BlockSize>::from_slice(in_block);
            let out_block = GenericArray::<u8, T::BlockSize>::from_mut_slice(out_block);

            self.alg.decrypt_block_b2b(in_block, out_block);
            out_block
                .iter_mut()
                .zip(buffer.iter())
                .for_each(|(b, i)| *b = *b ^ i);
            buffer.copy_from_slice(&in_block);
        }
        output
    }
}

impl<T: BlockEncrypt + BlockSizeUser> Cbc<T> {
    // Naive implementation only works for inputs that are N * blocksize
    pub fn encrypt(&self, plaintext: &[u8], iv: &[u8]) -> Vec<u8> {
        let mut output = vec![0; plaintext.len()];

        // This can probably be implemented more efficiently, by manually taking values from the iterator,
        // So that we can retian an refrence to the previous block to do the xor with
        let mut buffer = GenericArray::clone_from_slice(iv);
        for (in_block, out_block) in plaintext
            .chunks_exact(T::block_size())
            .zip(output.chunks_exact_mut(T::block_size()))
        {
            let in_block = GenericArray::<u8, T::BlockSize>::from_slice(in_block);
            let out_block = GenericArray::<u8, T::BlockSize>::from_mut_slice(out_block);

            buffer
                .iter_mut()
                .zip(in_block.iter())
                .for_each(|(b, i)| *b = *b ^ i);
            self.alg.encrypt_block(&mut buffer);

            out_block.copy_from_slice(&buffer);
        }
        output
    }
}
