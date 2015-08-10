use rustc_serialize::json::Json;

use client::WeChatClient;
use errors::WeChatError;

#[derive(Debug, Clone)]
pub struct WeChatUser<'a> {
    client: &'a WeChatClient,
}

impl<'a> WeChatUser<'a> {

    #[inline]
    pub fn new(client: &'a WeChatClient) -> WeChatUser<'a> {
        WeChatUser {
            client: client,
        }
    }

    pub fn get(&self, openid: &str) -> Result<Json, WeChatError> {
        self.client.get("user/info", vec![("openid", openid)])
    }

    pub fn get_with_lang(&self, openid: &str, lang: &str) -> Result<Json, WeChatError> {
        self.client.get("user/info", vec![("openid", openid), ("lang", lang)])
    }

    pub fn update_remark(&self, openid: &str, remark: &str) -> Result<Json, WeChatError> {
        let data = json!({
            "openid": (openid),
            "remark": (remark),
        });
        self.client.post("user/info/updateremark", vec![], &data)
    }

    pub fn get_followers(&self, next_openid: &str) -> Result<Json, WeChatError> {
        self.client.get("user/get", vec![("next_openid", next_openid)])
    }
}