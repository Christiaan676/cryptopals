use std::iter::FromIterator;
use std::ops::BitXor;

/// XOR two array of the same length
pub fn xor<const L: usize, T>(input_a: &[T; L], input_b: &[T; L]) -> Vec<T>
where
    T: BitXor + Copy,
    Vec<T>: FromIterator<<T>::Output>,
{
    input_a
        .iter()
        .zip(input_b.iter())
        .map(|(a, b)| *a ^ *b)
        .collect()
}

/// XOR an array with the given key
pub fn decode<T>(input: &[T], key: T) -> Vec<T>
where
    T: BitXor + Copy,
    Vec<T>: FromIterator<<T>::Output>,
{
    input.iter().map(|v| *v ^ key).collect()
}

pub fn decode_key<T>(input: &[T], key: &[T]) -> Vec<T>
where
    T: BitXor + Copy,
    Vec<T>: FromIterator<<T>::Output>,
{
    input
        .iter()
        .zip(key.iter().cycle())
        .map(|(a, b)| *a ^ *b)
        .collect()
}
