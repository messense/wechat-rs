use rustc_serialize::json::Json;

use client::WeChatClient;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Misc<'a> {
    client: &'a mut WeChatClient,
}

impl<'a> Misc<'a> {
    pub fn new(client: &'a mut WeChatClient) -> Misc<'a> {
        Misc {
            client: client,
        }
    }

    pub fn get_wechat_ips(&mut self) -> Vec<String> {
        let res = self.client.get("getcallbackip", vec![]);
        let data = match res {
            Ok(data) => data,
            Err(_) => { panic!("Error calling API"); },
        };
        let ip_list = data.find("ip_list").unwrap();
        let ip_array = ip_list.as_array().unwrap();
        let mut ips: Vec<String> = Vec::new();
        for v in ip_array.iter() {
            match v {
                &Json::String(ref ip) => {
                    ips.push(ip.to_owned());
                },
                _ => {},
            }
        }
        ips
    }
}
