// Constants for permutation tables
const P10_TABLE: [u8; 10] = [3, 5, 2, 7, 4, 10, 1, 9, 8, 6];
const P8_TABLE: [u8; 8] = [6, 3, 7, 4, 8, 5, 10, 9];

pub struct Key {
    pub k1: u8,
    pub k2: u8,
}

pub fn gen_keys() -> Vec<Key> {
    let mut keys = Vec::with_capacity(1024);

    for key in 0..1024 {
        keys.push(gen_subkeys(key));
    }

    keys
}

pub(crate) fn gen_subkeys(key: u16) -> Key {
    if key > 0b1111111111 {
        panic!("Key must be 10 bits or less");
    }

    // Apply initial permutation (P10) to the 10-bit key
    let mut permuted_key = 0;
    for &index in P10_TABLE.iter() {
        permuted_key <<= 1;
        permuted_key |= (key >> (10 - index)) & 1;
    }

    // Split the 10-bit key into two 5-bit halves
    let left_half = (permuted_key >> 5) as u8;
    let right_half = (permuted_key & 0b11111) as u8;

    // Perform a circular left shift on each half
    let left_shifted = ((left_half << 1) | (left_half >> 4)) & 0b11111;
    let right_shifted = ((right_half << 1) | (right_half >> 4)) & 0b11111;

    // Combine the two halves
    let combined = ((left_shifted as u16) << 5) | right_shifted as u16;

    // Apply permutation P8 to generate K1
    let mut k1: u8 = 0;
    for &index in P8_TABLE.iter() {
        k1 <<= 1;
        k1 |= ((combined >> (10 - index)) & 1) as u8;
    }

    // Perform another circular left shift on each half
    let left_shifted = ((left_shifted << 2) | (left_shifted >> 3)) & 0b11111;
    let right_shifted = ((right_shifted << 2) | (right_shifted >> 3)) & 0b11111;

    // Combine the two halves
    let combined = ((left_shifted as u16) << 5) | right_shifted as u16;

    // Apply permutation P8 to generate K2
    let mut k2: u8 = 0;
    for &index in P8_TABLE.iter() {
        k2 <<= 1;
        k2 |= ((combined >> (10 - index)) & 1) as u8;
    }

    Key { k1, k2 }
}