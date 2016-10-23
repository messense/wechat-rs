use std::io::Cursor;

use rand::thread_rng;
use rand::Rng;
use base64;
use byteorder::{NativeEndian, WriteBytesExt, ReadBytesExt};
use openssl::crypto::symm;

use types::WeChatResult;
use errors::WeChatError;


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

    pub fn encrypt(&self, plaintext: &str, _id: &str) -> WeChatResult<String> {
        let mut wtr = PrpCrypto::get_random_string().into_bytes();
        wtr.write_u32::<NativeEndian>((plaintext.len() as u32).to_be()).unwrap();
        wtr.extend(plaintext.bytes());
        wtr.extend(_id.bytes());
        // TODO: do not unwrap
        let encrypted = symm::encrypt(symm::Type::AES_256_CBC, &self.key, Some(&self.key[..16]), &wtr).unwrap();
        let b64encoded = base64::encode(&encrypted);
        Ok(b64encoded)
    }

    pub fn decrypt(&self, ciphertext: &str, _id: &str) -> WeChatResult<String> {
        let b64decoded = try!(base64::decode(ciphertext));
        // TODO: do not unwrap
        let text = symm::decrypt(symm::Type::AES_256_CBC, &self.key, Some(&self.key[..16]), &b64decoded).unwrap();
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
    use base64;
    use super::PrpCrypto;

    #[test]
    fn test_prpcrypto_encrypt() {
        let encoding_aes_key = "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR=";
        let key = base64::decode(encoding_aes_key).unwrap();
        let prp = PrpCrypto::new(&key);
        let encrypted = prp.encrypt("test", "rust").unwrap();
        assert_eq!("9s4gMv99m88kKTh/H8IdkNiFGeG9pd7vNWl50fGRWXY=", &encrypted);
    }

    #[test]
    fn test_prpcrypto_decrypt() {
        let encoding_aes_key = "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR=";
        let key = base64::decode(encoding_aes_key).unwrap();
        let prp = PrpCrypto::new(&key);
        let decrypted = prp.decrypt("9s4gMv99m88kKTh/H8IdkNiFGeG9pd7vNWl50fGRWXY=", "rust").unwrap();
        assert_eq!("test", &decrypted);
    }
}
