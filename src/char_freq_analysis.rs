use std::collections::HashMap;

use super::file_io::read_file_to_string;

pub fn calc_char_freq(data: &Vec<u8>) -> HashMap<u8, f32> {
    let mut freq_map = HashMap::new();

    // Count frequency of each byte
    for &byte in data {
        *freq_map.entry(byte).or_insert(0.0) += 1.0;
    }

    // Convert frequency to fraction of total
    for byte_freq in freq_map.values_mut() {
        *byte_freq /= data.len() as f32;
    }

    freq_map
}

pub fn calc_euclidean_dist(freq1: &HashMap<u8, f32>, freq2: &HashMap<u8, f32>) -> f32 {
    let mut sum = 0.0;

    // Add euclidean distance from all bytes present in freq1
    for (&byte, &freq) in freq1.iter() {
        let diff = freq - freq2.get(&byte).unwrap_or(&0.0);
        sum += diff * diff;
    }

    // Add all bytes present in freq2 but not freq1
    for (&byte, &freq) in freq2.iter() {
        if !freq1.contains_key(&byte) {
            sum += freq * freq;
        }
    }

    sum.sqrt()
}

pub fn freq_file_to_hashmap(file_path: &str) -> HashMap<u8, f32> {
    let contents = read_file_to_string(file_path);

    let mut freq_map = HashMap::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let byte = parts[0].parse::<u8>().unwrap();
        let freq = parts[1].parse::<f32>().unwrap();
        freq_map.insert(byte, freq);
    }

    freq_map
}