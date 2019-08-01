extern crate hex;

extern crate aes_soft as aes;
extern crate block_modes;

use aes::Aes128;
use rand::{Rng, OsRng};

use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn encrypt<'a>(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8>{
   let cipher = Aes128Cbc::new_var(&key, &iv).unwrap();
   cipher.encrypt_vec(data)
}

fn to_hex_string(bytes: Vec<u8>) -> String {
    bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .fold(String::new(), |string, hx| format!("{}{}", string, hx))
}

pub fn input_encryption(data: &mut String) -> String {
    let mut rng = OsRng::new().ok().unwrap();

    let mut iv: [u8; 16] = [0; 16];
    let mut key: [u8; 16] = [0; 16];

    rng.fill_bytes(&mut iv);
    rng.fill_bytes(&mut key);

    let encrypted = encrypt(data.as_bytes(), &key, &iv); 
    
    to_hex_string(encrypted)
}

pub fn input_decryption(data: &String) {
    // TODO: when returning, convert back for Vec<u8> to decrypt
    // let decimal: Vec<u8> = hex.iter().map(|hex| u8::from_str_radix(hex, 16).unwrap()).collect();
    // to decrypt: cipher.decrypt_vec(&ciphertext).unwrap()
    
    println!("{:?}", data);
    println!("{:?}", hex::decode(data));
}
