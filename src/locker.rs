extern crate crypto;
extern crate rand;

use std::str;

use crypto::{ symmetriccipher, buffer, aes, blockmodes  };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult  };

use rand::{ Rng, OsRng  };

// TODO: check this resource to better implement encryption: https://siciarz.net/24-days-of-rust-rust-crypto/
// All examples taken from: https://github.com/DaGenix/rust-crypto/blob/master/examples/symmetriccipher.rs

fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) 
    -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
        let mut encryptor = aes::cbc_encryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding
        );

        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(data);
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;

            final_result.extend(
                write_buffer
                    .take_read_buffer()
                    .take_remaining()
                    .iter()
                    .map(|&i| i)
            );

            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {}
            }
        }

        Ok(final_result)
    }

pub fn input_encryption(data: &mut String) -> String {
    let mut rng = OsRng::new().ok().unwrap();

    let mut iv: [u8; 16] = [0; 16];
    let mut key: [u8; 32] = [0; 32];

    rng.fill_bytes(&mut iv);
    rng.fill_bytes(&mut key);

    let encrypted = encrypt(data.as_bytes(), &key, &iv).ok().unwrap();

    // TODO: rust-crypto having issues for setting current 
    // return from encrypt to string, have to study
    // to see if another crate is better,
    // this one has currently no updates and
    // with some issues as to it being deprecated
    // a possible solution is this: https://github.com/RustCrypto
    println!("{:?}", data.as_bytes());
    println!("{:?}", encrypted);

    match str::from_utf8(&encrypted) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}
