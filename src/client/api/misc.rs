use rustc_serialize::json::Json;
use jsonway;

use types::WeChatResult;
use client::APIClient;
use session::SessionStore;


#[derive(Debug, Clone)]
pub struct WeChatMisc<T: SessionStore> {
    client: APIClient<T>,
}

impl<T: SessionStore> WeChatMisc<T> {

    #[inline]
    pub fn new(client: APIClient<T>) -> WeChatMisc<T> {
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
        let body = jsonway::object(|obj| {
            obj.set("action", "long2short".to_owned());
            obj.set("long_url", long_url.to_owned());
        }).unwrap();
        let data = try!(self.client.post("shorturl", vec![], body.as_object().unwrap()));
        let short = &data["short_url"];
        let short = short.as_string().unwrap();
        Ok(short.to_owned())
    }
}
