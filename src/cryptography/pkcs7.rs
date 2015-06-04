use std::char;

pub struct PKCS7Encoder;

impl PKCS7Encoder {
    pub fn encode(text: &str) -> String {
        const BLOCK_SIZE: u32 = 32u32;
        let length = text.len() as u32;
        let count = BLOCK_SIZE - length % BLOCK_SIZE;
        let padding_count = if count == 0 { BLOCK_SIZE } else { count };
        let padding = char::from_u32(padding_count).unwrap();
        let mut result = text.to_string();
        for _ in 0..padding_count {
            result.push(padding);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::PKCS7Encoder;

    #[test]
    fn test_pkcs7_encode() {
        let encoded = PKCS7Encoder::encode("test");
        assert_eq!(
            "test\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c",
            &encoded
        );
    }
}