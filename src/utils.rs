use std::time::{SystemTime, UNIX_EPOCH};
use hex::ToHex;
use openssl::crypto::hash;


pub fn check_signature<S: Into<String>, T: AsRef<str>>(token: S, signature: T, timestamp: S, nonce: S) -> bool {
    let mut data = vec![
        token.into(),
        timestamp.into(),
        nonce.into(),
    ];
    data.sort();
    let data_str = data.join("");
    // TODO: do not unwrap
    let real_sign = hash::hash(hash::Type::SHA1, data_str.as_bytes()).unwrap();
    signature.as_ref() == &real_sign.to_hex()
}

pub fn current_timestamp() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}

#[cfg(test)]
mod tests {
    use super::check_signature;

    #[test]
    fn test_check_signature_should_ok() {
        let token = "test";
        let signature = "f21891de399b4e33a1a93c9a7b8a8fffb5a443ff";
        let timestamp = "1410685589";
        let nonce = "test";
        assert!(check_signature(token, signature, timestamp, nonce));
    }

    #[test]
    fn test_check_signature_should_fail() {
        let token = "test";
        let signature = "f21891de399b4e33a1a93c9a7b8a8fffb5a443fe";
        let timestamp = "1410685589";
        let nonce = "test";
        assert!(!check_signature(token, signature, timestamp, nonce));
    }
}
