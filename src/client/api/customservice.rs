use std::io::Read;
use std::collections::HashMap;

use hex::ToHex;
use openssl::crypto::hash;
use jsonway;

use session::SessionStore;
use types::WeChatResult;
use client::APIClient;
use client::response::{KFAccount, OnlineKFAccount};


#[derive(Debug, Clone)]
pub struct WeChatCustomService<T: SessionStore> {
    client: APIClient<T>,
}

impl<T: SessionStore> WeChatCustomService<T> {

    #[inline]
    pub fn new(client: APIClient<T>) -> WeChatCustomService<T> {
        WeChatCustomService {
            client: client,
        }
    }

    pub fn add_account(&self, account: &str, nickname: &str, password: &str) -> WeChatResult<()> {
        // TODO: do not unwrap
        let encrypted_password = hash::hash(hash::Type::MD5, password.as_bytes()).unwrap();
        let encrypted_password = encrypted_password.to_hex();
        let data = jsonway::object(|obj| {
            obj.set("kf_account", account.to_owned());
            obj.set("nickname", nickname.to_owned());
            obj.set("password", encrypted_password);
        }).unwrap();
        try!(self.client.post(
            "https://api.weixin.qq.com/customservice/kfaccount/add",
            vec![],
            &data
        ));
        Ok(())
    }

    pub fn update_account(&self, account: &str, nickname: &str, password: &str) -> WeChatResult<()> {
        // TODO: do not unwrap
        let encrypted_password = hash::hash(hash::Type::MD5, password.as_bytes()).unwrap();
        let encrypted_password = encrypted_password.to_hex();
        let data = jsonway::object(|obj| {
            obj.set("kf_account", account.to_owned());
            obj.set("nickname", nickname.to_owned());
            obj.set("password", encrypted_password);
        }).unwrap();
        try!(self.client.post(
            "https://api.weixin.qq.com/customservice/kfaccount/update",
            vec![],
            &data
        ));
        Ok(())
    }

    pub fn delete_account(&self, account: &str) -> WeChatResult<()> {
        try!(self.client.get(
            "https://api.weixin.qq.com/customservice/kfaccount/del",
            vec![("kf_account", account)]
        ));
        Ok(())
    }

    pub fn get_accounts(&self) -> WeChatResult<Vec<KFAccount>> {
        let res = try!(self.client.get("customservice/getkflist", vec![]));
        let kf_list = &res["kf_list"];
        let kf_list = kf_list.as_array().unwrap();
        let mut accounts = vec![];
        for kf in kf_list {
            let kf_id = &kf["kf_id"];
            let kf_id = kf_id.as_string().unwrap();
            let kf_nick = &kf["kf_nick"];
            let kf_nick = kf_nick.as_string().unwrap();
            let kf_account = &kf["kf_account"];
            let kf_account = kf_account.as_string().unwrap();
            let avatar = &kf["kf_headimgurl"];
            let avatar = avatar.as_string().unwrap();
            let account = KFAccount {
                id: kf_id.to_owned(),
                nick: kf_nick.to_owned(),
                account: kf_account.to_owned(),
                avatar: avatar.to_owned(),
            };
            accounts.push(account);
        }
        Ok(accounts)
    }

    pub fn get_online_accounts(&self) -> WeChatResult<Vec<OnlineKFAccount>> {
        let res = try!(self.client.get("customservice/getonlinekflist", vec![]));
        let kf_list = &res["kf_online_list"];
        let kf_list = kf_list.as_array().unwrap();
        let mut accounts = vec![];
        for kf in kf_list {
            let kf_id = &kf["kf_id"];
            let kf_id = kf_id.as_string().unwrap();
            let kf_account = &kf["kf_account"];
            let kf_account = kf_account.as_string().unwrap();
            let status = &kf["status"];
            let status = status.as_u64().unwrap();
            let auto_accept = &kf["auto_accept"];
            let auto_accept = auto_accept.as_u64().unwrap();
            let accepted_case = &kf["accepted_case"];
            let accepted_case = accepted_case.as_u64().unwrap();
            let account = OnlineKFAccount {
                id: kf_id.to_owned(),
                account: kf_account.to_owned(),
                status: status,
                auto_accept: auto_accept,
                accepted_case: accepted_case,
            };
            accounts.push(account);
        }
        Ok(accounts)
    }

    pub fn upload_avatar<R: Read>(&self, account: &str, avatar: &mut R) -> WeChatResult<()> {
        let mut files = HashMap::new();
        files.insert("media".to_owned(), avatar);
        try!(
            self.client.upload_file(
                "https://api.weixin.qq.com/customservice/kfaccount/uploadheadimg",
                vec![("kf_account", account)],
                &mut files
            )
        );
        Ok(())
    }
}
