use rustc_serialize::Encodable;

use client::WeChatClient;
use errors::WeChatError;
use client::response::QRCodeTicket;

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
    
    pub fn create<D: Encodable>(&self, data: &D) -> Result<QRCodeTicket, WeChatError> {
        let res = try!(self.client.post("qrcode/create", vec![], data));
        let ticket = res.find("ticket").unwrap();
        let ticket = ticket.as_string().unwrap();
        let expire_seconds = match res.find("expire_seconds") {
            Some(seconds) => seconds.as_u64().unwrap(),
            None => 0u64,
        };
        let url = res.find("url").unwrap();
        let url = url.as_string().unwrap();
        Ok(QRCodeTicket {
            ticket: ticket.to_owned(),
            expire_seconds: expire_seconds as u32,
            url: url.to_owned(),
        })
    }

    pub fn get_url_with_ticket(ticket: &str) -> String {
        format!("https://mp.weixin.qq.com/cgi-bin/showqrcode?ticket={}", ticket)
    }

    pub fn get_url(qrcode_ticket: &QRCodeTicket) -> String {
        let ticket = &qrcode_ticket.ticket;
        Self::get_url_with_ticket(ticket)
    }
}
