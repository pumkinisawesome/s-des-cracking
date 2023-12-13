use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

/// Read the contents of a file into a string
pub fn read_file_to_string(file_path: &str) -> String {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file {file_path}: {}", e);
            exit(1);
        }
    };

    // read the file into a string
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error reading file {file_path}: {}", e);
            exit(1);
        }
    };

    data
}

/// Read the contents of a file into a vector of bytes
pub fn read_file_to_bytes(file_path: &str) -> Vec<u8> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file {file_path}: {}", e);
            exit(1);
        }
    };

    // read the file into a vector of bytes
    let mut data = Vec::new();
    match file.read_to_end(&mut data) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error reading file {file_path}: {}", e);
            exit(1);
        }
    };

    data
}

/// Write a vector of bytes to a file
pub fn write_bytes_to_file(file_path: &str, data: &[u8]) {
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
}