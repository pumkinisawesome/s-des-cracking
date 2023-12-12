mod key_gen;
mod crypt;

pub use key_gen::Key;
pub use key_gen::gen_keys;
pub(crate) use key_gen::gen_subkeys;
// pub use crypt::{encrypt, decrypt};