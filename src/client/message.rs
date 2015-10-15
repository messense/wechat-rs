use rustc_serialize::json::Json;

use client::WeChatClient;
use errors::WeChatError;


#[derive(Debug, Clone)]
pub struct WeChatMessage<'a> {
    client:  &'a WeChatClient,
}

impl<'a> WeChatMessage<'a> {

    #[inline]
    pub fn new(client: &'a WeChatClient) -> WeChatMessage<'a> {
        WeChatMessage {
            client: client,
        }
    }

    fn send_custom_message(&self, data: &mut Json, account: Option<&str>) -> Result<Json, WeChatError> {
        let mut data = data.as_object_mut().unwrap();
        if let Some(kf_account) = account {
            data.insert("kf_account".to_owned(), Json::String(kf_account.to_owned()));
        }
        self.client.post("message/custom/send", vec![], data)
    }

    pub fn send_text(&self, openid: &str, content: &str, account: Option<&str>) -> Result<Json, WeChatError> {
        let mut data = json!({
            "msgtype": "text",
            "touser": (openid),
            "text": {"content": (content)}
        });
        self.send_custom_message(&mut data, account)
    }

    pub fn send_image(&self, openid: &str, media_id: &str, account: Option<&str>) -> Result<Json, WeChatError> {
        let mut data = json!({
            "msgtype": "image",
            "touser": (openid),
            "image": {"media_id": (media_id)}
        });
        self.send_custom_message(&mut data, account)
    }

    pub fn send_voice(&self, openid: &str, media_id: &str, account: Option<&str>) -> Result<Json, WeChatError> {
        let mut data = json!({
            "msgtype": "voice",
            "touser": (openid),
            "voice": {"media_id": (media_id)}
        });
        self.send_custom_message(&mut data, account)
    }
}
