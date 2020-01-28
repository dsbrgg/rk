use std::str;

use aes_soft as aes;

use aes::Aes128;
use crypto_hash::{Algorithm, hex_digest};

use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

use crate::locker::{Bytes, ByteSize};

use ByteSize::*;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub struct Locker {
    iv: Bytes,
    key: Bytes,
    enc: Bytes,
}

impl Locker {
    // TODO: Keep encrypted data within structure
    // to make it easier to manipulate it without
    // having to read from disk

    pub fn new() -> Locker {
        let iv = Bytes::new_u16();
        let key = Bytes::new_u16();
        let enc = None;

        // TODO: store iv and key along with data to 
        // persist manipulating data further
        Locker {
            iv,
            key,
            enc
        }
    }

    fn encrypt(&self, data: &mut String) -> Vec<u8> {
        let bytes = data.as_bytes();

        Aes128Cbc::new_var(&self.key, &self.iv)
            .unwrap()
            .encrypt_vec(data)
    }

    fn decrypt(&self, data: &Vec<u8>) -> Vec<u8> {
        Aes128Cbc::new_var(&self.key, &self.iv)
           .unwrap()
           .decrypt_vec(data)
           .unwrap()
    }

    pub fn hash(&self, string: &str) -> String {
        hex_digest(
            Algorithm::SHA256, 
            string.as_bytes()
        )
    }

    // TODO: implement method to rotate key and iv
    // from within Locker instance

    pub fn input_encryption(&self, data: &mut String) -> String {
        Locker::bytes_to_hex(
            self.encrypt(data.as_bytes())
        )
    }

    pub fn input_decryption(&self, data: &String) -> String {
        let decoded = Locker::decode_hex(data);
        let binary = self.decrypt(&decoded); 

        str::from_utf8(&binary)
            .unwrap()
            .to_string()
    }
}
