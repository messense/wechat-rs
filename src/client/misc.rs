use rustc_serialize::json::Json;

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
            Err(_) => { panic!("Error calling API"); },
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
}
