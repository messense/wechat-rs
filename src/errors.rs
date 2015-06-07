use std::fmt;
use std::error;

use crypto::symmetriccipher::SymmetricCipherError;
use rustc_serialize::base64::FromBase64Error;

#[derive(Debug, Clone, Copy)]
pub enum WeChatError {
    InvalidSignature,
    InvalidAppId,
    SymmetricCipher(SymmetricCipherError),
    InvalidBase64(FromBase64Error),
}

impl fmt::Display for WeChatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WeChatError::InvalidSignature => write!(f, "Invalid signature"),
            WeChatError::InvalidAppId => write!(f, "Invalid app_id"),
            WeChatError::SymmetricCipher(ref err) => match *err {
                SymmetricCipherError::InvalidLength => write!(f, "Invalid length"),
                SymmetricCipherError::InvalidPadding => write!(f, "Invalid padding"),
            },
            WeChatError::InvalidBase64(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for WeChatError {
    fn description(&self) -> &str {
        match *self {
            WeChatError::InvalidSignature => "Invalid signature",
            WeChatError::InvalidAppId => "Invalid app_id",
            WeChatError::SymmetricCipher(ref err) => match *err {
                SymmetricCipherError::InvalidLength => "Invalid length",
                SymmetricCipherError::InvalidPadding => "Invalid padding",
            },
            WeChatError::InvalidBase64(ref err) => err.description(),
        }
    }
}