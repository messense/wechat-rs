use rustc_serialize::json::{Json, Object};

use client::WeChatClient;
use errors::WeChatError;

#[derive(Debug, Clone)]
pub struct WeChatMisc<'a> {
    client: &'a WeChatClient,
}

impl<'a> WeChatMisc<'a> {

    #[inline]
    pub fn new(client: &'a WeChatClient) -> WeChatMisc<'a> {
        WeChatMisc {
            client: client,
        }
    }

    pub fn get_wechat_ips(&self) -> Result<Vec<String>, WeChatError> {
        let data = try!(self.client.get("getcallbackip", vec![]));
        let ip_list = data.find("ip_list").unwrap();
        let ip_array = ip_list.as_array().unwrap();
        let mut ips: Vec<String> = Vec::new();
        for v in ip_array.iter() {
            if let &Json::String(ref ip) = v {
                ips.push(ip.to_owned());
            }
        }
        Ok(ips)
    }

    pub fn short_url(&self, long_url: &str) -> Result<String, WeChatError> {
        let mut body = Object::new();
        body.insert("action".to_owned(), Json::String("long2short".to_owned()));
        body.insert("long_url".to_owned(), Json::String(long_url.to_owned()));
        let data = try!(self.client.post("shorturl", vec![], &body));
        let short = data.find("short_url").unwrap();
        let short = short.as_string().unwrap();
        Ok(short.to_owned())
    }
}
