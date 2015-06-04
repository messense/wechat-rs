pub struct PrpCrypto {
    key: String,
}

impl PrpCrypto {
    pub fn new(key: &str) -> PrpCrypto {
        PrpCrypto {
            key: key.to_string(),
        }
    }
}