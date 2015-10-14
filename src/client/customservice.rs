use rustc_serialize::json::{Json, Array};
use openssl::crypto::hash;

use client::WeChatClient;
use errors::WeChatError;

#[derive(Debug, Clone)]
pub struct WeChatCustomService<'a> {
    client: &'a WeChatClient,
}

impl<'a> WeChatCustomService<'a> {

    #[inline]
    pub fn new(client: &'a WeChatClient) -> WeChatCustomService<'a> {
        WeChatCustomService {
            client: client,
        }
    }

    pub fn add_account(&self, account: &str, nickname: &str, password: &str) -> Result<Json, WeChatError> {
        let encrypted_password = hash::hash(hash::Type::MD5, password.as_bytes());
        let encrypted_password = String::from_utf8(encrypted_password).unwrap();
        let data  = json!({
            "kf_account": (account),
            "nickname": (nickname),
            "password": (encrypted_password)
        });
        self.client.post(
            "https://api.weixin.qq.com/customservice/kfaccount/add",
            vec![],
            &data
        )
    }

    pub fn update_account(&self, account: &str, nickname: &str, password: &str) -> Result<Json, WeChatError> {
        let encrypted_password = hash::hash(hash::Type::MD5, password.as_bytes());
        let encrypted_password = String::from_utf8(encrypted_password).unwrap();
        let data  = json!({
            "kf_account": (account),
            "nickname": (nickname),
            "password": (encrypted_password)
        });
        self.client.post(
            "https://api.weixin.qq.com/customservice/kfaccount/update",
            vec![],
            &data
        )
    }

    pub fn delete_account(&self, account: &str) -> Result<Json, WeChatError> {
        self.client.get(
            "https://api.weixin.qq.com/customservice/kfaccount/del",
            vec![("kf_account", account)]
        )
    }

    pub fn get_accounts(&self) -> Result<Array, WeChatError> {
        let res = try!(self.client.get("customservice/getkflist", vec![]));
        let kf_list = res.find("kf_list").unwrap();
        let kf_list = kf_list.as_array().unwrap();
        Ok(kf_list.clone())
    }

    pub fn get_online_accounts(&self) -> Result<Array, WeChatError> {
        let res = try!(self.client.get("customservice/getonlinekflist", vec![]));
        let kf_list = res.find("kf_online_list").unwrap();
        let kf_list = kf_list.as_array().unwrap();
        Ok(kf_list.clone())
    }
}
