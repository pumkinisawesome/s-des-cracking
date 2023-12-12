mod key_gen;
mod crypt;

pub use key_gen::gen_keys;
pub(crate) use key_gen::gen_subkeys;
pub use crypt::{encrypt, decrypt};

pub struct Key {
    pub k1: u8,
    pub k2: u8,
}