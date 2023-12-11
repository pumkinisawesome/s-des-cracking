use std::fs::File;
use std::io::{Read, Write};

fn main() {
    // load input.txt
    let file_path = "input.txt";
    let mut file = File::open(file_path).expect("File not found");

    // read the file into a vector of bytes
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("Something went wrong reading the file");

    // modify one byte, to make sure that this works
    contents[0] = b'J';

    // save to output.txt
    let mut output_file = File::create("output.txt").expect("File not found");
    output_file
        .write_all(&contents)
        .expect("Something went wrong writing the file");
}
