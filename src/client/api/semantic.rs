use rustc_serialize::json::Json;
use rustc_serialize::Encodable;

use types::WeChatResult;
use client::WeChatClient;
use session::SessionStore;


#[derive(Debug, Clone)]
pub struct WeChatSemantic<'a, T: SessionStore + 'a> {
    client: &'a WeChatClient<T>,
}

impl<'a, T: SessionStore> WeChatSemantic<'a, T> {

    #[inline]
    pub fn new(client: &'a WeChatClient<T>) -> WeChatSemantic<'a, T> {
        WeChatSemantic {
            client: client,
        }
    }

    pub fn search_simple(&self, query: &str, category: &str) -> WeChatResult<Json> {
        let body = json!({
            "query": (query),
            "category": (category),
            "appid": (self.client.appid)
        });
        self.search(&body)
    }

    pub fn search<D: Encodable>(&self, data: &D) -> WeChatResult<Json> {
        self.client.post("https://api.weixin.qq.com/semantic/semproxy/search", vec![], data)
    }
}
