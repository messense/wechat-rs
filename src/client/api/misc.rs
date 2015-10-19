use rustc_serialize::json::Json;

use session::SessionStore;
use client::{WeChatClient, WeChatResult};

#[derive(Debug, Clone)]
pub struct WeChatMisc<'a, T: SessionStore + 'a> {
    client: &'a WeChatClient<T>,
}

impl<'a, T: SessionStore> WeChatMisc<'a, T> {

    #[inline]
    pub fn new(client: &'a WeChatClient<T>) -> WeChatMisc<'a, T> {
        WeChatMisc {
            client: client,
        }
    }

    pub fn get_wechat_ips(&self) -> WeChatResult<Vec<String>> {
        let data = try!(self.client.get("getcallbackip", vec![]));
        let ip_list = &data["ip_list"];
        let ip_array = ip_list.as_array().unwrap();
        let mut ips: Vec<String> = Vec::new();
        for v in ip_array.iter() {
            if let &Json::String(ref ip) = v {
                ips.push(ip.to_owned());
            }
        }
        Ok(ips)
    }

    pub fn short_url(&self, long_url: &str) -> WeChatResult<String> {
        let body = json!({
            "action": "long2short",
            "long_url": (long_url)
        });
        let data = try!(self.client.post("shorturl", vec![], body.as_object().unwrap()));
        let short = &data["short_url"];
        let short = short.as_string().unwrap();
        Ok(short.to_owned())
    }
}
