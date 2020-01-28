use rand::{Rng, OsRng};

#[derive(Debug)]
pub enum ByteSize {
  U16,
  U32,
  U64
}

#[derive(Debug)]
pub struct Bytes {
    hex: String,
    binary: Vec<u8>
}

impl Bytes {
   
    /* Initializer */

    pub fn new(size: ByteSize) -> Bytes {
        let binary = match size {
            ByteSize::U16 => Bytes::random_u16(),
            ByteSize::U32 => Bytes::random_u32(),
            ByteSize::U64 => Bytes::random_u64(),
        };

        let hex = Bytes::bin_to_hex(&binary);

        Bytes {
            hex,
            binary
        }
    }

    pub fn from_bin(binary: Vec<u8>) -> Bytes {
        let hex = Bytes::bin_to_hex(&binary);

        Bytes {
            hex,
            binary
        }
    }

    pub fn from_hex(hex: String) -> Bytes {
        let binary = Bytes::hex_to_bin(&hex);

        Bytes {
            hex,
            binary
        }
    }

    /* Hex operations */
    
    pub fn hex(&self) -> String { self.hex.clone() }
    fn alloc_hex(&mut self, hex: String) {
        let binary = Bytes::hex_to_bin(&hex);

        self.hex = hex;
        self.binary = binary;
    }

    /* Binary operations */
    
    pub fn raw(&self) -> Vec<u8> { self.binary.clone() }
    fn alloc_raw(&mut self, binary: Vec<u8>) { 
        let hex = Bytes::bin_to_hex(&binary);

        self.hex = hex;
        self.binary = binary;
    }

    /* Associated functions */

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

    pub fn bin_to_hex(bytes: &Vec<u8>) -> String {
        bytes
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .fold(String::from("0x"), |string, hx| format!("{}{}", string, hx))
    }

    pub fn hex_to_bin(hex: &String) -> Vec<u8> {
        if !hex.is_empty() {
            return hex::decode(&hex[2..]).unwrap();
        }

        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ByteSize::*;

    #[test]
    fn new_u16() {
        let byte = Bytes::new(U16);

        assert_eq!(byte.raw().len(), 16);
    }

    #[test]
    fn new_u32() {
        let byte = Bytes::new(U32);

        assert_eq!(byte.raw().len(), 32);
    }

    #[test]
    fn new_u64() {
        let byte = Bytes::new(U64);

        assert_eq!(byte.raw().len(), 64);
    }

    #[test]
    fn from_bin() {
        let vec = [0].to_vec();
        let byte = Bytes::from_bin(vec);

        assert_eq!(byte.raw(), [0]);
        assert_eq!(byte.hex(), String::from("0x00"));
    }

    #[test]
    fn alloc_raw() {
        let vec = [0].to_vec();
        let other_vec = [1].to_vec();

        let mut byte = Bytes::from_bin(vec);
       
        assert_eq!(byte.raw(), [0]);
        assert_eq!(byte.hex(), String::from("0x00"));

        byte.alloc_raw(other_vec);

        assert_eq!(byte.raw(), [1]);
        assert_eq!(byte.hex(), String::from("0x01"));
    }

    #[test]
    fn alloc_hex() {
        let vec = [0].to_vec();
        let hex = String::from("0x01");

        let mut byte = Bytes::from_bin(vec);
       
        assert_eq!(byte.raw(), [0]);
        assert_eq!(byte.hex(), String::from("0x00"));

        byte.alloc_hex(hex);

        assert_eq!(byte.raw(), [1]);
        assert_eq!(byte.hex(), String::from("0x01"));
    }
}
