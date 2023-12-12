use std::fs::File;
use std::io::{Read, Write};
use std::collections::HashMap;

fn analyse_frequency(file_path: &str) -> std::io::Result<(HashMap<u8, usize>, usize)> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut frequency: HashMap<u8, usize> = HashMap::new();
    let mut total = 0;
    for byte in contents.as_bytes() {
        if byte.is_ascii() {
            *frequency.entry(*byte).or_insert(0) += 1;
            total += 1;
        }
    }
    Ok((frequency, total))
}

fn write_to_file(mut frequency: HashMap<u8, usize>, total: usize, file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;

    // Convert HashMap to Vec of tuples
    let mut frequency_vec: Vec<(u8, usize)> = frequency.drain().collect();

    // Sort Vec in descending order by frequency
    frequency_vec.sort_by(|a, b| b.1.cmp(&a.1));

    for (byte, count) in frequency_vec {
        let fraction = count as f64 / total as f64;
        writeln!(file, "{}:{:.6}", byte, fraction)?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let (frequency, total) = analyse_frequency("shakespeare.txt")?;
    write_to_file(frequency, total, "shakespeare_freq.txt")
}