use rustc_serialize::json::{Json, Object};

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
    
    pub fn create(&self, data: &Object) -> Result<Json, WeChatError> {
        self.client.post("qrcode/create", vec![], data)
    }
}