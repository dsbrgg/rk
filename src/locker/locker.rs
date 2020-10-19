
/* Dependencies */

use std::hash::{Hash, Hasher};

use aes_soft as aes;
use aes::Aes128;

use regex::Regex;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use crypto_hash::{Algorithm, hex_digest};

use crate::locker::{Bytes, ByteSize};

use ByteSize::*;

/* Custom types */

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

/* Distinguished struct */

#[derive(Clone, Debug)]
pub struct Distinguished {
    pub iv: String,
    pub key: String,
    pub dat: String,
    pub hash: String,
}

/* Encrypted struct */

#[derive(Clone, Debug)]
pub struct Encrypted(String);

/* Encrypted PartialEq behaviour */

impl PartialEq for Encrypted {
    fn eq(&self, other: &Self) -> bool {
        self.is_empty() == other.is_empty() || self.hash() == other.hash()
    }
}

/* Encrypted Eq behaviour */

impl Eq for Encrypted {}

/* Encrypted Hash behaviour */

impl Hash for Encrypted {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if !self.is_empty() {
            let Distinguished { hash, .. } = self.distinguish();

            hash.hash(state);
        }
    }
}

/* Encrypted behaviour */

impl Encrypted {

    /* Initialisers */

    pub fn new(iv: &str, key: &str, dat: &str, hash: &str) -> Encrypted {
        let value = format!(
            "{}${}${}${}",
            iv,
            key,
            dat,
            hash
        );

        Encrypted(value)
    }

    pub fn from(string: &str) -> Result<Encrypted, String> {
        let signature = Regex::new(r"^.*\$.*\$.*\$.*$").unwrap();

        if !signature.is_match(string) {
            return Err(format!("Malformed string signature: {}", string));
        }

        Ok(Encrypted(string.to_string()))
    }

    pub fn empty() -> Encrypted {
        Encrypted(String::new())
    }

    /* Methods */

    pub fn distinguish(&self) -> Distinguished {
        let split: Vec<&str> = self.0.split('$').collect();
        let iv = split[0].to_string();
        let key = split[1].to_string();
        let dat = split[2].to_string();
        let hash = split[3].to_string();

        Distinguished {
            iv,
            key,
            dat,
            hash
        }
    }

    pub fn hash(&self) -> String {
        let Distinguished { hash, .. } = self.distinguish();

        hash.to_string()
    }

    pub fn path(&self) -> String {
        self.0.clone() 
    }

    pub fn is_empty(&self) -> bool { 
        self.0.is_empty() 
    }
}

/* Locker struct */

#[derive(Debug)]
pub struct Locker {
    iv: Bytes,
    key: Bytes,
    pub dat: Bytes,
}

/* Locker struct behaviour */

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

    pub fn from(iv: String, key: String, dat: String) -> Locker {
        let iv = Bytes::from_hex(iv);
        let key = Bytes::from_hex(key);
        let dat = Bytes::from_hex(dat);

        Locker {
            iv,
            key,
            dat
        }
    }

    pub fn from_encrypted(encrypted: &Encrypted) -> Locker {
        let Distinguished { 
            iv, 
            key, 
            dat, 
            ..  
        } = encrypted.distinguish();

        Locker::from(iv, key, dat)
    }
    
    /* Methods */

    pub fn encrypt(&mut self, data: &str) -> Encrypted {
        if data.is_empty() {
            return Encrypted::empty();
        }

        let iv = self.iv.raw();
        let key = self.key.raw();
        let bytes = data.as_bytes();
        let encrypted = Aes128Cbc::new_var(&key[..], &iv[..])
            .unwrap()
            .encrypt_vec(bytes);

        self.dat.alloc_raw(encrypted);

        let iv = &self.iv.hex();
        let key = &self.key.hex();
        let dat = &self.dat.hex();
        let hash = Locker::hash(data);

        Encrypted::new(iv, key, dat, &hash)
    }

    pub fn decrypt(&self) -> String {
        let iv = self.iv.raw();
        let key = self.key.raw();
        let dat = self.dat.raw();

        let decrypted = Aes128Cbc::new_var(&key[..], &iv[..])
           .unwrap()
           .decrypt_vec(&dat[..])
           .unwrap();

        Bytes::bytes_string(&decrypted)
    }

    /* Associated functions */
    
    pub fn hash(string: &str) -> String {
        hex_digest(
            Algorithm::SHA256, 
            string.as_bytes()
        )
    }
}

/* Locker tests */

#[cfg(test)]
mod locker_tests {
    use super::*;

    #[test]
    fn new() {
        let locker = Locker::new();

        assert_eq!(locker.dat.raw().len(), 0);
        assert_eq!(locker.iv.raw().len(), 16);
        assert_eq!(locker.key.raw().len(), 16);
    }

    #[test]
    fn from() {
        let iv = String::from("0x00000000000000000000000000000000");
        let key = String::from("0x00000000000000000000000000000001");
        let dat = String::from("0x00000000000000000000000000000002");

        let iv_raw = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let key_raw = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let dat_raw = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];
        
        let locker = Locker::from(iv, key, dat);

        assert_eq!(locker.iv.raw(), iv_raw);
        assert_eq!(locker.key.raw(), key_raw);
        assert_eq!(locker.dat.raw(), dat_raw);
    }

    #[test]
    fn from_encrypted() {
        let iv = String::from("0x00000000000000000000000000000000");
        let key = String::from("0x00000000000000000000000000000001");
        let dat = String::from("0x00000000000000000000000000000002");
        let hash = String::from("0x00000000000000000000000000000003");

        let iv_raw = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let key_raw = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let dat_raw = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];

        let encrypted = Encrypted::new(&iv, &key, &dat, &hash);
        let locker = Locker::from_encrypted(&encrypted);

        assert_eq!(locker.iv.raw(), iv_raw);
        assert_eq!(locker.key.raw(), key_raw);
        assert_eq!(locker.dat.raw(), dat_raw);
    }

    #[test]
    fn encrypt() {
        let mut locker = Locker::new();
        let to_encrypt = "encrypt me!";
        let encrypted = locker.encrypt(to_encrypt);
        let formated = format!(
            "{}${}${}",
            locker.dat.hex(),
            &locker.iv.hex()[2..],
            &locker.key.hex()[2..]
        );

        assert_eq!(locker.dat.raw().len(), 16); 
        assert_eq!(locker.dat.hex().len(), 34); // Two extra bytes from 0x
        // assert_eq!(encrypted, formated);
    }

    #[test]
    fn decrypt() {
        let mut locker = Locker::new();
        let to_encrypt = "encrypt me!";

        locker.encrypt(to_encrypt);
        
        let decrypted = locker.decrypt();

        assert_eq!(decrypted, String::from("encrypt me!"));
    }

    #[test]
    fn hash() {
        let string = String::from("hash this");
        let hashed = String::from("19467788bc0cf11790a075ea718452cecf0e79db59d1964670475e5fe2e4a611");
        let hash = Locker::hash(&string);

        assert_eq!(hash, hashed);
    }
}

/* Encrypted tests */

#[cfg(test)]
mod encrypted_tests {
    use super::*;

    #[test]
    fn new() {
        let iv = "foo";
        let key = "bar";
        let dat = "biz";
        let hash = "fred";
        let encrypted = Encrypted::new(iv, key, dat, hash);

        assert_eq!(encrypted.0, "foo$bar$biz$fred");
    }

    #[test]
    fn from_pass() {
        let string = "foo$bar$biz$fred";
        let encrypted = Encrypted::from(string).unwrap();

        assert_eq!(encrypted.0, "foo$bar$biz$fred");
    }

    #[test]
    #[should_panic(expected = "Malformed string signature: foobar$biz$fred")]
    fn from_fail() {
        let string = "foobar$biz$fred";

        Encrypted::from(string).unwrap();
    }

    #[test]
    fn distinguish() {
        let encrypted = Encrypted::new("foo", "bar", "biz", "fred");
        let Distinguished { 
            iv,
            key,
            dat,
            hash
        } = encrypted.distinguish();

        assert_eq!(iv, "foo");
        assert_eq!(key, "bar");
        assert_eq!(dat, "biz");
        assert_eq!(hash, "fred");
    }

    #[test]
    fn partial_eq() {
        let foo = Encrypted::new("foo", "bar", "biz", "fred");
        let bar = Encrypted::new("bar", "foo", "biz", "fred");

        assert_eq!(foo, bar);
    }
}
