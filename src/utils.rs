use rustc_serialize::hex::ToHex;
use openssl::crypto::hash;


pub fn check_signature(token: &str, signature: &str, timestamp: i64, nonce: &str) -> bool {
    let mut data = vec![
        token.to_owned(),
        timestamp.to_string(),
        nonce.to_owned(),
    ];
    data.sort();
    let data_str = data.join("");
    let real_sign = hash::hash(hash::Type::SHA1, data_str.as_bytes());
    signature == &real_sign.to_hex()
}

#[cfg(test)]
mod tests {
    use super::check_signature;

    #[test]
    fn test_check_signature_should_ok() {
        let token = "test";
        let signature = "f21891de399b4e33a1a93c9a7b8a8fffb5a443ff";
        let timestamp = 1410685589;
        let nonce = "test";
        assert!(check_signature(token, signature, timestamp, nonce));
    }

    #[test]
    fn test_check_signature_should_fail() {
        let token = "test";
        let signature = "f21891de399b4e33a1a93c9a7b8a8fffb5a443fe";
        let timestamp = 1410685589;
        let nonce = "test";
        assert!(!check_signature(token, signature, timestamp, nonce));
    }
}
