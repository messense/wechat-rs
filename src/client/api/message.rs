use rustc_serialize::Encodable;

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

    pub fn send<D: Encodable>(&self, data: &D) -> Result<(), WeChatError> {
        try!(self.client.post("message/custom/send", vec![], data));
        Ok(())
    }

    pub fn send_text(&self, openid: &str, content: &str) -> Result<(), WeChatError> {
        use client::request::SendTextRequest;

        let req = SendTextRequest::new(openid, content);
        self.send(&req)
    }

    pub fn send_image(&self, openid: &str, media_id: &str) -> Result<(), WeChatError> {
        use client::request::SendImageRequest;

        let req = SendImageRequest::new(openid, media_id);
        self.send(&req)
    }

    pub fn send_voice(&self, openid: &str, media_id: &str) -> Result<(), WeChatError> {
        use client::request::SendVoiceRequest;

        let req = SendVoiceRequest::new(openid, media_id);
        self.send(&req)
    }
}
