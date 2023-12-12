use super::Key;

const IP_TABLE: [u8; 8] = [2, 6, 3, 1, 4, 8, 5, 7];
const INVERSE_IP_TABLE: [u8; 8] = [4, 1, 3, 5, 7, 2, 8, 6];
const EP_TABLE: [u8; 8] = [4, 1, 2, 3, 2, 3, 4, 1];
const P4_TABLE: [u8; 4] = [2, 4, 3, 1];

const S0_BOX: [[u8; 4]; 4] = [
    [0b01, 0b00, 0b11, 0b10],
    [0b11, 0b10, 0b01, 0b00],
    [0b00, 0b10, 0b01, 0b11],
    [0b11, 0b01, 0b11, 0b10],
];
const S1_BOX: [[u8; 4]; 4] = [
    [0b00, 0b01, 0b10, 0b11],
    [0b10, 0b00, 0b01, 0b11],
    [0b11, 0b00, 0b01, 0b00],
    [0b10, 0b01, 0b00, 0b11],
];

pub fn encrypt(data: &[u8], key: &Key) -> Vec<u8> {
    crypt(data, key.k1, key.k2)
}

pub fn decrypt(data: &[u8], key: &Key) -> Vec<u8> {
    crypt(data, key.k2, key.k1)
}

fn crypt(data: &[u8], k1: u8, k2: u8) -> Vec<u8> {
    let mut output = Vec::with_capacity(data.len());

    for byte in data {
        // Apply initial permutation (IP) to the byte
        let mut permuted_byte = 0;
        for &index in IP_TABLE.iter() {
            permuted_byte <<= 1;
            permuted_byte |= (byte >> (8 - index)) & 1;
        }

        // Split the byte into two 4-bit halves
        let left = permuted_byte >> 4;
        let right = permuted_byte & 0b1111;

        // Perform the first Feistel round with k1
        let (new_left, new_right) = feistel_round(left, right, k1);

        // Perform the second Feistel round with k2
        let (final_left, final_right) = feistel_round(new_right, new_left, k2);

        // Combine the final halves and apply the inverse of IP
        let combined = (final_left << 4) | final_right;
        let mut permuted_byte = 0;
        for &index in INVERSE_IP_TABLE.iter() {
            permuted_byte <<= 1;
            permuted_byte |= (combined >> (8 - index)) & 1;
        }

        output.push(permuted_byte);
    }

    output
}

fn feistel_round(left: u8, right: u8, key: u8) -> (u8, u8) {
    // Apply expansion permutation (EP) to the right half
    let mut expanded_right = 0;
    for &index in EP_TABLE.iter() {
        expanded_right <<= 1;
        expanded_right |= (right >> (4 - index)) & 1;
    }

    // XOR the expanded right half with the key
    let xor_result = expanded_right ^ key;

    // Split the result into two 4-bit halves
    let xor_left = xor_result >> 4;
    let xor_right = xor_result & 0b1111;

    // Apply the S-boxes to the halves
    let s0_result = apply_sbox(xor_left, &S0_BOX);
    let s1_result = apply_sbox(xor_right, &S1_BOX);

    // Combine the S-box results and apply the P4 permutation
    let sbox_result = (s0_result << 2) | s1_result;
    let mut p4_result = 0;
    for &index in P4_TABLE.iter() {
        p4_result <<= 1;
        p4_result |= (sbox_result >> (4 - index)) & 1;
    }

    // XOR the P4 result with the left half and swap the halves
    let new_right = right;
    let new_left = left ^ p4_result;

    (new_left, new_right)
}

fn apply_sbox(input: u8, sbox: &[[u8; 4]; 4]) -> u8 {
    let row = ((input & 0b1000) >> 2) | (input & 1);
    let col = (input & 0b0110) >> 1;
    sbox[row as usize][col as usize]
}