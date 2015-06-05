use rand::thread_rng;
use rand::Rng;
use rustc_serialize::base64;
use rustc_serialize::base64::ToBase64;
use byteorder::{NativeEndian, WriteBytesExt};

use super::pkcs7::PKCS7Encoder;
use super::aes;

pub struct PrpCrypto {
    key: String,
}

impl PrpCrypto {
    pub fn new(key: &str) -> PrpCrypto {
        PrpCrypto {
            key: key.to_string(),
        }
    }

    fn get_random_string() -> String {
        if cfg!(test) {
            "1234567890123456".to_string()
        } else {
            thread_rng().gen_ascii_chars().take(16).collect()
        }
    }

    pub fn encrypt(&self, text: &str, _id: &str) -> String {
        let mut wtr = PrpCrypto::get_random_string().into_bytes();
        wtr.write_u32::<NativeEndian>(text.len().to_be() as u32).unwrap();
        wtr.extend(text.bytes());
        wtr.extend(_id.bytes());
        let encoded = PKCS7Encoder::encode(wtr);
        let encrypted = aes::encrypt(&encoded, self.key.as_bytes(), &self.key.as_bytes()[0..16])
            .ok()
            .expect("AES encrypt failed");
        let b64encoded = encrypted.to_base64(base64::STANDARD);
        b64encoded
    }
}

#[cfg(test)]
mod tests {
    use super::PrpCrypto;
}