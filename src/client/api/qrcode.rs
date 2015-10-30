use rustc_serialize::Encodable;

use types::WeChatResult;
use client::APIClient;
use client::response::QRCodeTicket;
use session::SessionStore;


#[derive(Debug, Clone)]
pub struct WeChatQRCode<T: SessionStore> {
    client: APIClient<T>,
}

impl<T: SessionStore> WeChatQRCode<T> {

    #[inline]
    pub fn new(client: APIClient<T>) -> WeChatQRCode<T> {
        WeChatQRCode {
            client: client,
        }
    }
    
    pub fn create<D: Encodable>(&self, data: &D) -> WeChatResult<QRCodeTicket> {
        let res = try!(self.client.post("qrcode/create", vec![], data));
        let ticket = &res["ticket"];
        let ticket = ticket.as_string().unwrap();
        let expire_seconds = match res.find("expire_seconds") {
            Some(seconds) => seconds.as_u64().unwrap(),
            None => 0u64,
        };
        let url = &res["url"];
        let url = url.as_string().unwrap();
        Ok(QRCodeTicket {
            ticket: ticket.to_owned(),
            expire_seconds: expire_seconds as u32,
            url: url.to_owned(),
        })
    }

    pub fn get_url_with_ticket(&self, ticket: &str) -> String {
        format!("https://mp.weixin.qq.com/cgi-bin/showqrcode?ticket={}", ticket)
    }

    pub fn get_url(&self, qrcode_ticket: &QRCodeTicket) -> String {
        let ticket = &qrcode_ticket.ticket;
        self.get_url_with_ticket(ticket)
    }
}
