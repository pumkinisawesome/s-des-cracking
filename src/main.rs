#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

mod s_des;
mod entropy;

use s_des::{gen_keys, gen_subkeys};
use entropy::calc_entropy;

fn main() {
    let keys = gen_keys();

    for key in &keys {
        println!("k1: {:#010b}, k2: {:#010b}", key.k1, key.k2);
    }

    println!("Total keys generated: {}", keys.len());

    let my_key = gen_subkeys(0b1101001101);

    println!("My key:");
    println!("k1: {:#010b}, k2: {:#010b}", my_key.k1, my_key.k2);
}