use rustc_serialize::json::{Json, ToJson};

use client::WeChatClient;
use errors::WeChatError;

#[derive(Debug, Clone)]
pub struct WeChatQRCode<'a> {
    client: &'a WeChatClient,
}

impl<'a> WeChatQRCode<'a> {

    #[inline]
    pub fn new(client: &'a WeChatClient) -> WeChatQRCode<'a> {
        WeChatQRCode {
            client: client,
        }
    }
    
    pub fn create<D: ToJson>(&self, data: &D) -> Result<Json, WeChatError> {
        self.client.post("qrcode/create", vec![], &data.to_json())
    }

    pub fn get_url_with_ticket(ticket: &str) -> String {
        format!("https://mp.weixin.qq.com/cgi-bin/showqrcode?ticket={}", ticket)
    }

    pub fn get_url(qrcode_res: &Json) -> String {
        let ticket = qrcode_res.find("ticket").unwrap().as_string().unwrap();
        Self::get_url_with_ticket(&ticket)
    }
}
