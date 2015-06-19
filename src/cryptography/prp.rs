use std::io::Cursor;

use rand::thread_rng;
use rand::Rng;
use rustc_serialize::base64;
use rustc_serialize::base64::{FromBase64, ToBase64};
use byteorder::{NativeEndian, WriteBytesExt, ReadBytesExt};

use errors::WeChatError;
use cryptography::aes;

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
            "1234567890123456".to_owned()
        } else {
            thread_rng().gen_ascii_chars().take(16).collect()
        }
    }

    pub fn encrypt(&self, plaintext: &str, _id: &str) -> Result<String, WeChatError> {
        let mut wtr = PrpCrypto::get_random_string().into_bytes();
        wtr.write_u32::<NativeEndian>((plaintext.len() as u32).to_be()).unwrap();
        wtr.extend(plaintext.bytes());
        wtr.extend(_id.bytes());
        let encrypted = match aes::encrypt(&wtr, &self.key, &self.key[..16]) {
            Ok(val) => val,
            Err(err) => return Err(WeChatError::SymmetricCipher(err)),
        };
        let b64encoded = encrypted.to_base64(base64::STANDARD);
        Ok(b64encoded)
    }

    pub fn decrypt(&self, ciphertext: &str, _id: &str) -> Result<String, WeChatError> {
        let b64decoded = match ciphertext.from_base64() {
            Ok(val) => val,
            Err(err) => return Err(WeChatError::InvalidBase64(err)),
        };
        let text = match aes::decrypt(&b64decoded, &self.key, &self.key[..16]) {
            Ok(val) => val,
            Err(err) => return Err(WeChatError::SymmetricCipher(err)),
        };
        let mut rdr = Cursor::new(text[16..20].to_vec());
        let content_length = u32::from_be(rdr.read_u32::<NativeEndian>().unwrap()) as usize;
        let content = &text[20 .. content_length + 20];
        let from_id = &text[content_length + 20 ..];
        if from_id != _id.as_bytes() {
            return Err(WeChatError::InvalidAppId);
        }
        let content_string = String::from_utf8(content.to_vec()).unwrap();
        Ok(content_string)
    }
}

#[cfg(test)]
mod tests {
    use rustc_serialize::base64::FromBase64;
    use super::PrpCrypto;

    #[test]
    fn test_prpcrypto_encrypt() {
        let encoding_aes_key = "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR=";
        let key = encoding_aes_key.from_base64().unwrap();
        let prp = PrpCrypto::new(&key);
        let encrypted = prp.encrypt("test", "rust").unwrap();
        assert_eq!("9s4gMv99m88kKTh/H8IdkNiFGeG9pd7vNWl50fGRWXY=", &encrypted);
    }

    #[test]
    fn test_prpcrypto_decrypt() {
        let encoding_aes_key = "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR=";
        let key = encoding_aes_key.from_base64().unwrap();
        let prp = PrpCrypto::new(&key);
        let decrypted = prp.decrypt("9s4gMv99m88kKTh/H8IdkNiFGeG9pd7vNWl50fGRWXY=", "rust").unwrap();
        assert_eq!("test", &decrypted);
    }
}