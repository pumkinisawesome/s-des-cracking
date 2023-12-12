use std::fs::File;
use std::io::{Read, Write, Result};
use std::process::exit;

pub fn read_file_to_bytes(file_path: &str) -> Result<Vec<u8>> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            exit(1);
        }
    };

    // read the file into a vector of bytes
    let mut data = Vec::new();
    match file.read_to_end(&mut data) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            exit(1);
        }
    };

    Ok(data)
}

pub fn write_bytes_to_file(file_path: &str, data: &[u8]) -> Result<()> {
    let mut output_file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            exit(1);
        }
    };
    match output_file.write_all(data) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error writing to file: {}", e);
            exit(1);
        }
    };

    Ok(())
}