use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello, world!");

    let file_path = "src/input_test.txt";
    let mut file = File::open(file_path).expect("File not found");

    // read the file into a vector of bytes
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("Something went wrong reading the file");

    // Calculate the entropy of the file
    let entropy = calc_entropy(&contents);

    // Print the entropy
    println!("Entropy: {}", entropy);
}

// function to determine information entropy of a vector of bytes
fn calc_entropy(data: &[u8]) -> f64 {
    // create a hash map to store the frequency of each byte
    let mut freq_map = HashMap::new();

    // count the frequency of each byte
    for &byte in data {
        let count = freq_map.entry(byte).or_insert(0);
        *count += 1;
    }

    // calculate the entropy
    let mut entropy = 0.0;
    let data_len = data.len() as f64;
    for value in freq_map.values() {
        let freq = *value as f64 / data_len;
        entropy -= freq * freq.log2();
    }

    entropy
}