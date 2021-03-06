use hex::ToHex;
use base64;
use openssl::crypto::hash;

mod prp;

use errors::WeChatError;
use types::WeChatResult;
use self::prp::PrpCrypto;


#[derive(Debug, Eq, PartialEq)]
pub struct WeChatCrypto {
    token: String,
    key: Vec<u8>,
    _id: String,
}

impl WeChatCrypto {
    pub fn new(token: &str, encoding_aes_key: &str, _id: &str) -> WeChatCrypto {
        let mut aes_key = encoding_aes_key.to_owned();
        aes_key.push('=');
        let key = base64::decode(&aes_key).unwrap();
        WeChatCrypto {
            token: token.to_owned(),
            key: key,
            _id: _id.to_owned(),
        }
    }

    fn get_signature(&self, timestamp: i64, nonce: &str, encrypted: &str) -> String {
        let mut data = vec![
            self.token.clone(),
            timestamp.to_string(),
            nonce.to_owned(),
            encrypted.to_owned(),
        ];
        data.sort();
        let data_str = data.join("");
        // TODO: do not unwrap
        let signature = hash::hash(hash::Type::SHA1, data_str.as_bytes()).unwrap();
        signature.to_hex()
    }

    pub fn check_signature(&self, signature: &str, timestamp: i64, nonce: &str, echo_str: &str) -> WeChatResult<String> {
        let real_signature = self.get_signature(timestamp, nonce, echo_str);
        if signature != &real_signature {
            return Err(WeChatError::InvalidSignature);
        }
        let prp = PrpCrypto::new(&self.key);
        let msg = try!(prp.decrypt(echo_str, &self._id));
        Ok(msg)
    }

    pub fn encrypt_message(&self, msg: &str, timestamp: i64, nonce: &str) -> WeChatResult<String> {
        let prp = PrpCrypto::new(&self.key);
        let encrypted_msg = try!(prp.encrypt(msg, &self._id));
        let signature = self.get_signature(timestamp, nonce, &encrypted_msg);
        let msg = format!(
            "<xml>\n\
            <Encrypt><![CDATA[{encrypt}]]></Encrypt>\n\
            <MsgSignature><![CDATA[{signature}]]></MsgSignature>\n\
            <TimeStamp>{timestamp}</TimeStamp>\n\
            <Nonce><![CDATA[{nonce}]]></Nonce>\n\
            </xml>",
            encrypt=encrypted_msg,
            signature=signature,
            timestamp=timestamp,
            nonce=nonce,
        );
        Ok(msg)
    }

    pub fn decrypt_message(&self, xml: &str, signature: &str, timestamp: i64, nonce: &str) -> WeChatResult<String> {
        use super::xmlutil;
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let encrypted_msg = xmlutil::evaluate(&doc, "//xml/Encrypt/text()").string();
        let real_signature = self.get_signature(timestamp, nonce, &encrypted_msg);
        if signature != &real_signature {
            return Err(WeChatError::InvalidSignature);
        }
        let prp = PrpCrypto::new(&self.key);
        let msg = try!(prp.decrypt(&encrypted_msg, &self._id));
        Ok(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::WeChatCrypto;

    #[test]
    fn test_get_signature() {
        let crypto = WeChatCrypto::new("test", "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR", "test");
        let signature = crypto.get_signature(123456i64, "test", "rust");
        assert_eq!("d6056f2bb3ad3e30f4afa5ef90cc9ddcdc7b7b27", &signature);
    }

    #[test]
    fn test_check_signature_should_ok() {
        let signature = "dd6b9c95b495b3f7e2901bfbc76c664930ffdb96";
        let timestamp = 1411443780;
        let nonce = "437374425";
        let echo_str = "4ByGGj+sVCYcvGeQYhaKIk1o0pQRNbRjxybjTGblXrBaXlTXeOo1+bXFXDQQb1o6co6Yh9Bv41n7hOchLF6p+Q==";
        let crypto = WeChatCrypto::new("123456", "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR", "wx49f0ab532d5d035a");
        match crypto.check_signature(signature, timestamp, nonce, echo_str) {
            Ok(_) => {},
            Err(_) => panic!("Check signature failed"),
        }
    }

    #[test]
    #[should_panic]
    fn test_check_signature_should_fail() {
        let signature = "dd6b9c95b495b3f7e2901bfbc76c664930ffdb96";
        let timestamp = 1411443780;
        let nonce = "437374424";
        let echo_str = "4ByGGj+sVCYcvGeQYhaKIk1o0pQRNbRjxybjTGblXrBaXlTXeOo1+bXFXDQQb1o6co6Yh9Bv41n7hOchLF6p+Q==";
        let crypto = WeChatCrypto::new("123456", "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR", "wx49f0ab532d5d035a");
        match crypto.check_signature(signature, timestamp, nonce, echo_str) {
            Ok(_) => {},
            Err(_) => panic!("Check signature failed"),
        }
    }

    #[test]
    fn test_encrypt_message() {
        let timestamp = 1411525903;
        let nonce = "461056294";
        let msg = "<xml>\n\
            <MsgType><![CDATA[text]]></MsgType>\n\
            <Content><![CDATA[test]]></Content>\n\
            <FromUserName><![CDATA[wx49f0ab532d5d035a]]></FromUserName>\n\
            <ToUserName><![CDATA[messense]]></ToUserName>\n\
            <AgentID>1</AgentID>\n\
            <CreateTime>1411525903</CreateTime>\n\
            </xml>";
        let expected = "<xml>\n\
            <Encrypt><![CDATA[9s4gMv99m88kKTh/H8IdkOiMg6bisoy3ypwy9H4hvSPe9nsGaqyw5hhSjdYbcrKk+j3nba4HMOTzHrluLBYqxgNcBqGsL8GqxlhZgURnAtObvesEl5nZ+uBE8bviY0LWke8Zy9V/QYKxNV2FqllNXcfmstttyIkMKCCmVbCFM2JTF5wY0nFhHZSjPUL2Q1qvSUCUld+/WIXrx0oyKQmpB6o8NRrrNrsDf03oxI1p9FxUgMnwKKZeOA/uu+2IEvEBtb7muXsVbwbgX05UPPJvFurDXafG0RQyPR+mf1nDnAtQmmNOuiR5MIkdQ39xn1vWwi1O5oazPoQJz0nTYjxxEE8kv3kFxtAGVRe3ypD3WeK2XeFYFMNMpatF9XiKzHo3]]></Encrypt>\n\
            <MsgSignature><![CDATA[407518b7649e86ef23978113f92d27afa9296533]]></MsgSignature>\n\
            <TimeStamp>1411525903</TimeStamp>\n\
            <Nonce><![CDATA[461056294]]></Nonce>\n\
            </xml>";
        let crypto = WeChatCrypto::new("123456", "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR", "wx49f0ab532d5d035a");
        let encrypted = crypto.encrypt_message(msg, timestamp, nonce).unwrap();
        assert_eq!(expected, &encrypted);
    }

    #[test]
    fn test_decrypt_message() {
        let xml = "<xml><ToUserName><![CDATA[wx49f0ab532d5d035a]]></ToUserName>\n\
            <Encrypt><![CDATA[RgqEoJj5A4EMYlLvWO1F86ioRjZfaex/gePD0gOXTxpsq5Yj4GNglrBb8I2BAJVODGajiFnXBu7mCPatfjsu6IHCrsTyeDXzF6Bv283dGymzxh6ydJRvZsryDyZbLTE7rhnus50qGPMfp2wASFlzEgMW9z1ef/RD8XzaFYgm7iTdaXpXaG4+BiYyolBug/gYNx410cvkKR2/nPwBiT+P4hIiOAQqGp/TywZBtDh1yCF2KOd0gpiMZ5jSw3e29mTvmUHzkVQiMS6td7vXUaWOMZnYZlF3So2SjHnwh4jYFxdgpkHHqIrH/54SNdshoQgWYEvccTKe7FS709/5t6NMxuGhcUGAPOQipvWTT4dShyqio7mlsl5noTrb++x6En749zCpQVhDpbV6GDnTbcX2e8K9QaNWHp91eBdCRxthuL0=]]></Encrypt>\n\
            <AgentID><![CDATA[1]]></AgentID>\n\
            </xml>";
        let expected = "<xml><ToUserName><![CDATA[wx49f0ab532d5d035a]]></ToUserName>\n\
            <FromUserName><![CDATA[messense]]></FromUserName>\n\
            <CreateTime>1411525903</CreateTime>\n\
            <MsgType><![CDATA[text]]></MsgType>\n\
            <Content><![CDATA[test]]></Content>\n\
            <MsgId>4363689963896700987</MsgId>\n\
            <AgentID>1</AgentID>\n\
            </xml>";

        let signature = "74d92dfeb87ba7c714f89d98870ae5eb62dff26d";
        let timestamp = 1411525903;
        let nonce = "461056294";
        let crypto = WeChatCrypto::new("123456", "kWxPEV2UEDyxWpmPdKC3F4dgPDmOvfKX1HGnEUDS1aR", "wx49f0ab532d5d035a");
        let decrypted = crypto.decrypt_message(xml, signature, timestamp, nonce).unwrap();
        assert_eq!(expected, &decrypted);
    }
}
