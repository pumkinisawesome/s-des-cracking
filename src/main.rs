#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

mod s_des;
mod entropy;
mod file_io;

use s_des::{gen_keys, gen_subkeys, encrypt, decrypt, Key};
use entropy::calc_entropy;
use file_io::{read_file_to_bytes, write_bytes_to_file};

fn main() {
    // let keys = gen_keys();

    // for key in &keys {
    //     println!("k1: {:#010b}, k2: {:#010b}", key.k1, key.k2);
    // }

    // println!("Total keys generated: {}", keys.len());

    // let my_key = gen_subkeys(0b1101001101).unwrap();

    // println!("My key:");
    // println!("k1: {:#010b}, k2: {:#010b}", my_key.k1, my_key.k2);

    // Test encryption
    let byte_to_encrypt = 0b10101010;
    println!("plain byte: {:#010b}", byte_to_encrypt);

    let my_key = Key {
        k1: 0b11011010,
        k2: 0b10001101,
    };

    let encrypted = s_des::encrypt(&[byte_to_encrypt], &my_key);
    println!("Encrypted:  {:#010b}", encrypted[0]);

    let decrypted = s_des::decrypt(&encrypted, &my_key);
    println!("Decrypted:  {:#010b}", decrypted[0]);

    println!("Alrighty, that worked! Now let's encrypt and decrypt a file. ");

    // Load input_test.txt in as vector of bytes
    let input_file_path = "input_test.txt";
    let input_data = read_file_to_bytes(input_file_path).unwrap();
    println!("Read {} bytes from {}", input_data.len(), input_file_path);

    // Encrypt the data
    let encrypted_data = encrypt(&input_data, &my_key);
    println!("Encrypted {} bytes", encrypted_data.len());

    // Calculate the entropy of the encrypted data
    let entropy = calc_entropy(&encrypted_data);
    println!("Entropy: {}", entropy);

    // Write the encrypted data to a file
    let encrypted_file_path = "encrypted_test.txt";
    write_bytes_to_file(encrypted_file_path, &encrypted_data).unwrap();

    // Decrypt the data
    let decrypted_data = decrypt(&encrypted_data, &my_key);
    println!("Decrypted {} bytes", decrypted_data.len());

    // Calculate the entropy of the decrypted data
    let entropy = calc_entropy(&decrypted_data);
    println!("Entropy: {}", entropy);

    // Write the decrypted data to a file
    let decrypted_file_path = "decrypted_test.txt";
    write_bytes_to_file(decrypted_file_path, &decrypted_data).unwrap();
}