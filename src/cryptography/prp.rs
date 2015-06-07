use std::io::Cursor;

use rand::thread_rng;
use rand::Rng;
use rustc_serialize::base64;
use rustc_serialize::base64::{FromBase64, ToBase64};
use byteorder::{NativeEndian, WriteBytesExt, ReadBytesExt};

use super::aes;

#[derive(Debug, Eq, PartialEq)]
pub struct PrpCrypto {
    key: Vec<u8>,
}

impl PrpCrypto {
    pub fn new(key: &[u8]) -> PrpCrypto {
        PrpCrypto {
            key: key.to_vec(),
        }
    }

    fn get_random_string() -> String {
        if cfg!(test) {
            "1234567890123456".to_string()
        } else {
            thread_rng().gen_ascii_chars().take(16).collect()
        }
    }

    pub fn encrypt(&self, plaintext: &str, _id: &str) -> String {
        let mut wtr = PrpCrypto::get_random_string().into_bytes();
        wtr.write_u32::<NativeEndian>((plaintext.len() as u32).to_be()).unwrap();
        wtr.extend(plaintext.bytes());
        wtr.extend(_id.bytes());
        let encrypted = aes::encrypt(&wtr, &self.key, &self.key[..16])
            .ok()
            .expect("AES encrypt failed");
        let b64encoded = encrypted.to_base64(base64::STANDARD);
        b64encoded
    }

    pub fn decrypt(&self, ciphertext: &str, _id: &str) -> String {
        let b64decoded = ciphertext.from_base64()
            .ok()
            .expect("Base64 decode error");
        let text = aes::decrypt(&b64decoded, &self.key, &self.key[..16])
            .ok()
            .expect("AES decrypt failed");
        let mut rdr = Cursor::new(text[16..20].to_vec());
        let content_length = u32::from_be(rdr.read_u32::<NativeEndian>().unwrap()) as usize;
        let content = &text[20 .. content_length + 20];
        let from_id = &text[content_length + 20 ..];
        String::from_utf8(content.to_vec()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use rustc_serialize::base64::FromBase64;
    use super::PrpCrypto;

    #[test]
    fn test_prpcrypto_encrypt() {
        let encoding_aes_key = "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR=";
        let key = encoding_aes_key.from_base64().ok().expect("Base64 decode error");
        let prp = PrpCrypto::new(&key);
        let encrypted = prp.encrypt("test", "rust");
        assert_eq!("9s4gMv99m88kKTh/H8IdkNiFGeG9pd7vNWl50fGRWXY=", &encrypted);
    }

    #[test]
    fn test_prpcrypto_decrypt() {
        let encoding_aes_key = "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR=";
        let key = encoding_aes_key.from_base64().ok().expect("Base64 decode error");
        let prp = PrpCrypto::new(&key);
        let decrypted = prp.decrypt("9s4gMv99m88kKTh/H8IdkNiFGeG9pd7vNWl50fGRWXY=", "rust");
        assert_eq!("test", &decrypted);
    }
}