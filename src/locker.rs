use std::str;

extern crate hex;

extern crate aes_soft as aes;
extern crate block_modes;

use aes::Aes128;
use rand::{Rng, OsRng};

use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub struct Locker { 
    iv: [u8; 16],
    key: [u8; 16],
}

impl Locker {
    pub fn new() -> Locker {
        let mut rng = OsRng::new().ok().unwrap();

        let mut iv: [u8; 16] = [0; 16];
        let mut key: [u8; 16] = [0; 16];

        rng.fill_bytes(&mut iv);
        rng.fill_bytes(&mut key);

        // TODO: store iv and key along with data to 
        // persist manipulating data further
        Locker {
            iv,
            key,
        }
    }

    fn to_hex_string(bytes: Vec<u8>) -> String {
        bytes
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .fold(String::from("0x"), |string, hx| format!("{}{}", string, hx))
    }

    fn encrypt<'a>(&self, data: &[u8]) -> Vec<u8>{
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
        Locker::to_hex_string(
            self.encrypt(data.as_bytes())
        )
    }

    pub fn input_decryption(&self, data: &String) -> String {
        let decoded = match data.starts_with("0x") {
            true => hex::decode(&data[2..]).unwrap(),
            false => panic!("Wrong hex format in decrytion!"),
        };

        let binary = self.decrypt(&decoded); 

        str::from_utf8(&binary)
            .unwrap()
            .to_string()
    }
}

