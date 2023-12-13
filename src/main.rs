use rand::Rng;

mod s_des;
mod file_io;
mod char_freq_analysis;

use crate::s_des::{gen_keys, encrypt, decrypt, Key};
use crate::file_io::{read_file_to_bytes, write_bytes_to_file};
use crate::char_freq_analysis::{
    freq_file_to_hashmap,
    calc_char_freq,
    calc_euclidean_dist
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        match args[1].as_str() {
            "gen" => gen_test_file("plaintext"),
            "crack" => crack_file("encrypted"),
            _ => print_usage(),
        }
    } else if args.len() == 3 {
        match args[1].as_str() {
            "gen" => gen_test_file(&args[2]),
            "crack" => crack_file(&args[2]),
            _ => print_usage(),
        }
    } else {
        print_usage();

    }
}

fn print_usage() {
    println!("Usage:\tcargo run [gen|crack] <filename>");
    println!();
    println!("\tgen:\tgenerates an encrypted file with a random key");
    println!("\t\tfilename defaults to plaintext");
    println!("\tcrack:\tcracks an encrypted file with brute force");
    println!("\t\tfilename defaults to encrypted");
}

/// Generates an encrypted file with a random key
fn gen_test_file(file_name: &str) {
    let file_path = format!("data/{}.txt", file_name);

    println!("Generating test file");

    // Generate random key
    let mut rng = rand::thread_rng();
    let key = rng.gen_range(0..1024);
    let key = s_des::gen_subkeys(key).unwrap();
    println!("Key:\n\tk1: {:#010b}, \n\tk2: {:#010b}", key.k1, key.k2);

    let input_data = read_file_to_bytes(&file_path);
    println!("{} bytes read from {}", input_data.len(), file_path);

    let encrypted_data = encrypt(&input_data, &key);

    let file_path = "data/encrypted.txt";
    write_bytes_to_file(file_path, &encrypted_data);
    println!("{} bytes saved to {}", encrypted_data.len(), file_path);
}

/// Cracks an encrypted file with brute force
fn crack_file(file_name: &str) {
    let file_path = format!("data/{}.txt", file_name);

    println!("Cracking file");

    let input_data = read_file_to_bytes(&file_path);
    println!("{} bytes read from {}", input_data.len(), file_path);

    // Prepare character frequency of English from Shakespeare
    let shakespeare_file_path = "resources/shakespeare_freq.txt";
    let shakespeare_char_freq = freq_file_to_hashmap(shakespeare_file_path);

    let keys = gen_keys();

    let mut lowest_distance: Option<f32> = None;
    let mut best_key: Option<&Key> = None;
    for key in &keys {
        let decrypted_data = decrypt(&input_data, key);

        let char_freq = calc_char_freq(&decrypted_data);

        let distance = calc_euclidean_dist(&char_freq, &shakespeare_char_freq);

        if lowest_distance.is_none() || distance < lowest_distance.unwrap() {
            lowest_distance = Some(distance);
            best_key = Some(key);
        }
    }

    let best_key = best_key.unwrap();

    println!("Best key:");
    println!("\tk1: {:010b}, \n\tk2: {:010b}", best_key.k1, best_key.k2);

    let decrypted_data = decrypt(&input_data, best_key);

    let output_file_path = "data/decrypted.txt";
    write_bytes_to_file(output_file_path, &decrypted_data);
    println!("{} bytes saved to {}", decrypted_data.len(), output_file_path);
}