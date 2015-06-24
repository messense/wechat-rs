use rustc_serialize::json::{Json, Object};

use client::WeChatClient;

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

    pub fn get_wechat_ips(&self) -> Vec<String> {
        let res = self.client.get("getcallbackip", vec![]);
        let data = match res {
            Ok(data) => data,
            Err(_) => { return vec![]; },
        };
        let ip_list = data.find("ip_list").unwrap();
        let ip_array = ip_list.as_array().unwrap();
        let mut ips: Vec<String> = Vec::new();
        for v in ip_array.iter() {
            if let &Json::String(ref ip) = v {
                ips.push(ip.to_owned());
            }
        }
        ips
    }

    pub fn short_url(&self, long_url: &str) -> String {
        let mut body = Object::new();
        body.insert("action".to_owned(), Json::String("long2short".to_owned()));
        body.insert("long_url".to_owned(), Json::String(long_url.to_owned()));
        let res = self.client.post("shorturl", vec![], &body);
        let data = match res {
            Ok(data) => data,
            Err(_) => { return "".to_owned() },
        };
        let short = data.find("short_url").unwrap();
        let short = short.as_string().unwrap();
        short.to_owned()
    }
}
