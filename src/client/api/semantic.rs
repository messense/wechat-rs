use rustc_serialize::json::Json;
use rustc_serialize::Encodable;

use types::WeChatResult;
use client::APIClient;
use session::SessionStore;


#[derive(Debug, Clone)]
pub struct WeChatSemantic<T: SessionStore> {
    client: APIClient<T>,
}

impl<T: SessionStore> WeChatSemantic<T> {

    #[inline]
    pub fn new(client: APIClient<T>) -> WeChatSemantic<T> {
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
