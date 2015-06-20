use std::collections::HashMap;

use hyper::Client;
use rustc_serialize::json;

pub struct WeChatClient {
    pub appid: String,
    pub secret: String,
    pub access_token: String,
}

impl WeChatClient {
	pub fn new(appid: &str, secret: &str) -> WeChatClient {
		WeChatClient {
			appid: appid.to_owned(),
			secret: secret.to_owned(),
			access_token: "".to_owned(),
		}
	}

	pub fn with_access_token(appid: &str, secret: &str, access_token: &str) -> WeChatClient {
		WeChatClient {
			appid: appid.to_owned(),
			secret: secret.to_owned(),
			access_token: access_token.to_owned(),
		}
	}
}