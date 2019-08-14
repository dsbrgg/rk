use std::str;

extern crate hex;

extern crate aes_soft as aes;
extern crate block_modes;

use aes::Aes128;
use rand::{Rng, OsRng};

use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub struct Locker<'l> {
    // TODO: use enums here to maintain structure
    // concise
    iv: [u8; 16],
    key: [u8; 16],
    iv0x: String,
    key0x: String,
    iv_path: &'l str,
    key_path: &'l str,
}

impl<'l> Locker<'l> {
    pub fn new() -> Locker<'l> {
        let mut rng = OsRng::new().ok().unwrap();

        let mut iv: [u8; 16] = [0; 16];
        let mut key: [u8; 16] = [0; 16];

        rng.fill_bytes(&mut iv);
        rng.fill_bytes(&mut key);

        let iv0x = Locker::bytes_to_hex(iv.to_vec());
        let key0x = Locker::bytes_to_hex(key.to_vec());

        // TODO: separate this logic when reading from old registers
        // to be able to instantiate another Locker isntance
        let ivAgain = Locker::hex_to_bytes(&iv0x);

        // TODO: store iv and key along with data to 
        // persist manipulating data further
        Locker {
            iv,
            key,
            iv0x,
            key0x,
            iv_path: "iv",
            key_path: "key"
        }
    }

    pub fn from_u8(iv: [u8; 16], key: [u8; 16]) -> Locker<'l> {
        let iv0x = Locker::bytes_to_hex(iv.to_vec());
        let key0x = Locker::bytes_to_hex(key.to_vec());

         Locker {
            iv,
            key,
            iv0x,
            key0x,
            iv_path: "iv",
            key_path: "key"
        }
    }

    pub fn from_hex(iv0x: String, key0x: String) -> Locker<'l> {
        let iv = Locker::hex_to_bytes(&iv0x);
        let key = Locker::hex_to_bytes(&key0x);

         Locker {
            iv,
            key,
            iv0x,
            key0x,
            iv_path: "iv",
            key_path: "key"
        }
    }

    fn decode_hex(hx: &String) -> Vec<u8> {
        match hx.starts_with("0x") {
            true => hex::decode(&hx[2..]).unwrap(),
            false => panic!("Wrong hex format when parsing to bytes!"),
        }
    }

    fn hex_to_bytes(hx: &String) -> [u8; 16] {
        let decoded = Locker::decode_hex(hx); 

        if decoded.len() < 16 { panic!("Wrong hex format!"); } 

        [
            decoded[0],  decoded[1],  decoded[2],  decoded[3],
            decoded[4],  decoded[5],  decoded[6],  decoded[7],
            decoded[8],  decoded[9],  decoded[10], decoded[11],
            decoded[12], decoded[13], decoded[14], decoded[15],
        ]
    }

    fn bytes_to_hex(bytes: Vec<u8>) -> String {
        bytes
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .fold(String::from("0x"), |string, hx| format!("{}{}", string, hx))
    }

    fn encrypt<'a>(&self, data: &[u8]) -> Vec<u8> {
        Aes128Cbc::new_var(&self.key, &self.iv)
            .unwrap()
            .encrypt_vec(data)
    }

    fn decrypt<'a>(&self, data: &Vec<u8>) -> Vec<u8> {
       Aes128Cbc::new_var(&self.key, &self.iv)
           .unwrap()
           .decrypt_vec(data)
           .unwrap()
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

