use std::str;

use aes_soft as aes;

use aes::Aes128;

use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use crypto_hash::{Algorithm, hex_digest};

use crate::locker::{Bytes, ByteSize};

use ByteSize::*;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

#[derive(Debug)]
pub struct Locker {
    iv: Bytes,
    key: Bytes,
    pub dat: Bytes,
}

impl Locker {

    /* Initialisers */

    // TODO: implement different byte sizes
    pub fn new() -> Locker {
        let dat = Bytes::new(E);
        let iv = Bytes::new(U16);
        let key = Bytes::new(U16);

        Locker {
            iv,
            key,
            dat
        }
    }

    /* Methods */

    fn encrypt(&mut self, data: &mut String) {
        let iv = self.iv.raw();
        let key = self.key.raw();
        let bytes = data.as_bytes();

        let encrypted = Aes128Cbc::new_var(&key[..], &iv[..])
            .unwrap()
            .encrypt_vec(bytes);

        self.dat.alloc_raw(encrypted);
    }

    fn decrypt(&self) -> String {
        let iv = self.iv.raw();
        let key = self.key.raw();
        let dat = self.dat.raw();

        let decrypted = Aes128Cbc::new_var(&key[..], &iv[..])
           .unwrap()
           .decrypt_vec(&dat[..])
           .unwrap();

        str::from_utf8(&decrypted)
            .unwrap()
            .to_string()
    }

    pub fn hash(&self, string: &str) -> String {
        hex_digest(
            Algorithm::SHA256, 
            string.as_bytes()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let locker = Locker::new();

        assert_eq!(locker.dat.raw().len(), 0);
        assert_eq!(locker.iv.raw().len(), 16);
        assert_eq!(locker.key.raw().len(), 16);
    }

    #[test]
    fn encrypt() {
        let mut locker = Locker::new();
        let mut to_encrypt = String::from("encrypt me!");

        locker.encrypt(&mut to_encrypt);

        assert_eq!(locker.dat.raw().len(), 16); 
        assert_eq!(locker.dat.hex().len(), 34); // Two extra bytes from 0x
    }

    #[test]
    fn decrypt() {
        let mut locker = Locker::new();
        let mut to_encrypt = String::from("encrypt me!");

        locker.encrypt(&mut to_encrypt);
        
        let decrypted = locker.decrypt();

        assert_eq!(decrypted, String::from("encrypt me!"));
    }

    #[test]
    fn hash() {
        let locker = Locker::new();

        let string = String::from("hash this");
        let hashed = String::from("19467788bc0cf11790a075ea718452cecf0e79db59d1964670475e5fe2e4a611");

        let hash = locker.hash(&string);

        assert_eq!(hash, hashed);
    }
}
