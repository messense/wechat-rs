use rustc_serialize::json::Json;

use client::WeChatClient;
use errors::WeChatError;

#[derive(Debug, Clone)]
pub struct WeChatSemantic<'a> {
    client: &'a WeChatClient,
}

impl<'a> WeChatSemantic<'a> {

    #[inline]
    pub fn new(client: &'a WeChatClient) -> WeChatSemantic<'a> {
        WeChatSemantic {
            client: client,
        }
    }

    pub fn search_simple(&self, query: &str, category: &str) -> Result<Json, WeChatError> {
        let body = json!({
            "query": (query),
            "category": (category),
            "appid": (self.client.appid)
        });
        self.search(&body)
    }

    pub fn search(&self, data: &Json) -> Result<Json, WeChatError> {
        let res = try!(self.client.post("https://api.weixin.qq.com/semantic/semproxy/search", vec![], data));
        Ok(res)
    }
}