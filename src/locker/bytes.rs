use std::str;
use rand::{Rng, OsRng};

// TODO: create Encrypted struct 

#[derive(Debug)]
pub enum ByteSize {
    E,
    U16,
    U32,
    U64
}

#[derive(Debug)]
pub struct Bytes {
    size: ByteSize,
    hex: String,
    binary: Vec<u8>
}

impl Bytes {
   
    /* Initializer */

    pub fn new(size: ByteSize) -> Bytes {
        let binary = match size {
            ByteSize::E => Bytes::empty(),
            ByteSize::U16 => Bytes::random_u16(),
            ByteSize::U32 => Bytes::random_u32(),
            ByteSize::U64 => Bytes::random_u64(),
        };

        let hex = Bytes::bin_to_hex(&binary);

        Bytes {
            size,
            hex,
            binary
        }
    }

    pub fn from_bin(binary: Vec<u8>) -> Bytes {
        let size = match binary.len() {
            0 => ByteSize::E,
            16 => ByteSize::U16,
            32 => ByteSize::U32,
            64 => ByteSize::U64,
            _ => panic!("Invalid vec length!"),
        };

        let hex = Bytes::bin_to_hex(&binary);

        Bytes {
            size,
            hex,
            binary
        }
    }

    pub fn from_hex(hex: String) -> Bytes {
        let size = match &hex[2..].len() {
            0 => ByteSize::E,
            32 => ByteSize::U16,
            64 => ByteSize::U32,
            128 => ByteSize::U64,
            _ => panic!("Invalid hex length!"),
        };
        
        let binary = Bytes::hex_to_bin(&hex);

        Bytes {
            size,
            hex,
            binary
        }
    }

    pub fn rotate(self) -> Bytes { Bytes::new(self.size) }

    /* Hex operations */
    
    pub fn hex(&self) -> String { self.hex.clone() }
    pub fn alloc_hex(&mut self, hex: String) {
        let binary = Bytes::hex_to_bin(&hex);

        self.hex = hex;
        self.binary = binary;
    }

    /* Binary operations */
    
    pub fn raw(&self) -> Vec<u8> { self.binary.clone() }
    pub fn alloc_raw(&mut self, binary: Vec<u8>) { 
        let hex = Bytes::bin_to_hex(&binary);

        self.hex = hex;
        self.binary = binary;
    }

    /* Associated functions */

    fn empty() -> Vec<u8> { Vec::new() }

    fn random_u16() -> Vec<u8> {
        let mut rng = OsRng::new().ok().unwrap();
        let mut random_bytes: [u8; 16] = [0; 16];

        rng.fill_bytes(&mut random_bytes);

        random_bytes.to_vec()
    }

    fn random_u32() -> Vec<u8> {
        let mut rng = OsRng::new().ok().unwrap();
        let mut random_bytes: [u8; 32] = [0; 32];

        rng.fill_bytes(&mut random_bytes); 

        random_bytes.to_vec()
    } 

    fn random_u64() -> Vec<u8> {
        let mut rng = OsRng::new().ok().unwrap();
        let mut random_bytes: [u8; 64] = [0; 64];

        rng.fill_bytes(&mut random_bytes);

        random_bytes.to_vec()
    }

    pub fn bytes_string(string: &[u8]) -> String {
        str::from_utf8(&string)
            .unwrap()
            .to_string()
    }

    pub fn bin_to_hex(bytes: &Vec<u8>) -> String {
        bytes
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .fold(String::from("0x"), |string, hx| format!("{}{}", string, hx))
    }

    pub fn hex_to_bin(hex: &String) -> Vec<u8> {
        if hex.is_empty() || !hex.starts_with("0x") {
            panic!("Wrong hex format!");
        }

        hex::decode(&hex[2..]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ByteSize::*;

    #[test]
    fn new_empty() {
        let mut byte = Bytes::new(E);
        let empty_vec: Vec<u8> = Vec::new();

        assert_eq!(byte.raw(), empty_vec);
        assert_eq!(byte.hex(), String::from("0x"));
    }

    #[test]
    fn new_u16() {
        let byte = Bytes::new(U16);

        assert_eq!(byte.raw().len(), 16);
        assert_eq!(byte.hex().len(), 34); // Two extra bytes from 0x
    }

    #[test]
    fn new_u32() {
        let byte = Bytes::new(U32);

        assert_eq!(byte.raw().len(), 32);
        assert_eq!(byte.hex().len(), 66); // Two extra bytes from 0x
    }

    #[test]
    fn new_u64() {
        let byte = Bytes::new(U64);

        assert_eq!(byte.raw().len(), 64);
        assert_eq!(byte.hex().len(), 130); // Two extra bytes from 0x
    }

    #[test]
    fn from_bin() {
        let vec = [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ].to_vec();
        let vec_string = String::from("0x00000000000000000000000000000000");
        
        let byte = Bytes::from_bin(vec.clone());

        assert_eq!(byte.raw(), vec);
        assert_eq!(byte.hex(), vec_string);
    }

    #[test]
    #[should_panic(expected = "Invalid vec length!")]
    fn from_bin_panic() {
        let vec = [ 0, 0, 0 ].to_vec();
        
        Bytes::from_bin(vec);
    } 

    #[test]
    fn from_hex() {
        let vec = [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ];
        let hex = String::from("0x00000000000000000000000000000001");

        let mut byte = Bytes::from_hex(hex.clone());

        assert_eq!(byte.raw(), vec);
        assert_eq!(byte.hex(), hex);
    }

    #[test]
    #[should_panic(expected = "Invalid hex length!")]
    fn from_hex_panic() {
        let hex = String::from("0x00");
        
        Bytes::from_hex(hex);
    }

    #[test]
    fn rotate() {
        let mut byte = Bytes::new(U16);

        byte = byte.rotate();

        assert_eq!(byte.raw().len(), 16);
        assert_eq!(byte.hex().len(), 34); // Two extra bytes from 0x
    }

    #[test]
    fn alloc_raw() {
        let vec = [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ].to_vec();
        let other_vec = [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ].to_vec();

        let vec_string = String::from("0x00000000000000000000000000000000");
        let other_vec_string = String::from("0x00000000000000000000000000000001");

        let mut byte = Bytes::from_bin(vec.clone());
       
        assert_eq!(byte.raw(), vec);
        assert_eq!(byte.hex(), vec_string);

        byte.alloc_raw(other_vec.clone());

        assert_eq!(byte.raw(), other_vec);
        assert_eq!(byte.hex(), other_vec_string);
    }

    #[test]
    fn alloc_hex() {
        let hex_bin = [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ].to_vec();
        let hex = String::from("0x00000000000000000000000000000001");
        
        let vec = [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ].to_vec();
        let vec_string = String::from("0x00000000000000000000000000000000");

        let mut byte = Bytes::from_bin(vec.clone());
       
        assert_eq!(byte.raw(), vec);
        assert_eq!(byte.hex(), vec_string);

        byte.alloc_hex(hex.clone());

        assert_eq!(byte.raw(),hex_bin);
        assert_eq!(byte.hex(), hex);
    }  

    #[test]
    #[should_panic(expected = "Wrong hex format!")]
    fn from_hex_panic_format() {
        let hex = String::from("00");
        
        Bytes::from_hex(hex);
    }
}
