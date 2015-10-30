use rustc_serialize::Encodable;

use types::WeChatResult;
use client::APIClient;
use session::SessionStore;


#[derive(Debug, Clone)]
pub struct WeChatMessage<T: SessionStore> {
    client:  APIClient<T>,
}

impl<T: SessionStore> WeChatMessage<T> {

    #[inline]
    pub fn new(client: APIClient<T>) -> WeChatMessage<T> {
        WeChatMessage {
            client: client,
        }
    }

    pub fn send<D: Encodable>(&self, data: &D) -> WeChatResult<()> {
        try!(self.client.post("message/custom/send", vec![], data));
        Ok(())
    }

    pub fn send_text(&self, openid: &str, content: &str) -> WeChatResult<()> {
        use client::request::SendTextRequest;

        let req = SendTextRequest::new(openid, content);
        self.send(&req)
    }

    pub fn send_image(&self, openid: &str, media_id: &str) -> WeChatResult<()> {
        use client::request::SendImageRequest;

        let req = SendImageRequest::new(openid, media_id);
        self.send(&req)
    }

    pub fn send_voice(&self, openid: &str, media_id: &str) -> WeChatResult<()> {
        use client::request::SendVoiceRequest;

        let req = SendVoiceRequest::new(openid, media_id);
        self.send(&req)
    }
}
