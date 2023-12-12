use std::collections::HashMap;

/// function to determine information entropy of a vector of bytes
///
pub fn calc_entropy(data: &[u8]) -> f64 {
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