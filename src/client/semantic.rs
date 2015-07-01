use rustc_serialize::json::{Json, Object};
use rustc_serialize::Encodable;

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
        let mut body = Object::new();
        body.insert("query".to_owned(), Json::String(query.to_owned()));
        body.insert("category".to_owned(), Json::String(category.to_owned()));
        body.insert("appid".to_owned(), Json::String(self.client.appid.clone()));
        self.search(&mut body)
    }

    pub fn search<D: Encodable>(&self, data: &D) -> Result<Json, WeChatError> {
        // data.insert("appid".to_owned(), Json::String(self.client.appid.clone()));
        let res = try!(self.client.post("https://api.weixin.qq.com/semantic/semproxy/search", vec![], &data));
        Ok(res)
    }
}