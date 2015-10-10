use std::fmt;
use std::error;

use rustc_serialize::base64::FromBase64Error;

#[derive(Debug, Clone)]
pub enum WeChatError {
    InvalidSignature,
    InvalidAppId,
    InvalidBase64(FromBase64Error),
    ClientError { errcode: i32, errmsg: String },
}

impl fmt::Display for WeChatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WeChatError::InvalidSignature => write!(f, "Invalid signature"),
            WeChatError::InvalidAppId => write!(f, "Invalid app_id"),
            WeChatError::InvalidBase64(ref err) => err.fmt(f),
            WeChatError::ClientError { errcode, ref errmsg } => write!(f, "Client error code: {}, message: {}", errcode, errmsg),
        }
    }
}

impl error::Error for WeChatError {

    #[allow(unused_variables)]
    fn description(&self) -> &str {
        match *self {
            WeChatError::InvalidSignature => "Invalid signature",
            WeChatError::InvalidAppId => "Invalid app_id",
            WeChatError::InvalidBase64(ref err) => err.description(),
            WeChatError::ClientError { errcode, ref errmsg } => errmsg,
        }
    }
}
