use std::collections::HashMap;

use rustc_serialize::json::Json;
use jsonway;

use types::WeChatResult;
use client::APIClient;
use client::response::{User, Followers};
use session::SessionStore;


#[derive(Debug, Clone)]
pub struct WeChatUser<T: SessionStore> {
    client: APIClient<T>,
}

impl<T: SessionStore> WeChatUser<T> {

    #[inline]
    pub fn new(client: APIClient<T>) -> WeChatUser<T> {
        WeChatUser {
            client: client,
        }
    }

    pub fn get(&self, openid: &str) -> WeChatResult<User> {
        self.get_with_lang(openid, "zh_CN")
    }

    pub fn get_with_lang(&self, openid: &str, lang: &str) -> WeChatResult<User> {
        let res = try!(self.client.get("user/info", vec![("openid", openid), ("lang", lang)]));
        Ok(self.json_to_user(&res))
    }

    pub fn update_remark(&self, openid: &str, remark: &str) -> WeChatResult<()> {
        let data = jsonway::object(|obj| {
            obj.set("openid", openid.to_owned());
            obj.set("remark", remark.to_owned());
        }).unwrap();
        try!(self.client.post("user/info/updateremark", vec![], &data));
        Ok(())
    }

    pub fn get_followers(&self, next_openid: Option<&str>) -> WeChatResult<Followers> {
        let params = match next_openid {
            Some(openid) => vec![("next_openid", openid)],
            None => vec![],
        };
        let res = try!(self.client.get("user/get", params));
        let total = &res["total"];
        let total = total.as_u64().unwrap();
        let count = &res["count"];
        let count = count.as_u64().unwrap();
        let next_id = &res["next_openid"];
        let next_id = next_id.as_string().unwrap();
        let openids = match res.find_path(&["data", "openid"]) {
            Some(data) => {
                let openids_array = data.as_array().unwrap();
                openids_array.iter()
                             .map(|x| x.as_string().unwrap().to_owned())
                             .collect::<Vec<_>>()
            },
            None => vec![],
        };
        Ok(Followers {
            total: total,
            count: count,
            openids: openids,
            next_openid: next_id.to_owned(),
        })
    }

    pub fn get_group_id(&self, openid: &str) -> WeChatResult<u64> {
        let data = jsonway::object(|obj| {
            obj.set("openid", openid.to_owned());
        }).unwrap();
        let res = try!(self.client.post("groups/getid", vec![], &data));
        let group_id = &res["groupid"];
        let group_id = group_id.as_u64().unwrap();
        Ok(group_id)
    }

    fn json_to_user(&self, res: &Json) -> User {
        let _subscribe = &res["subscribe"];
        let subscribe = match _subscribe.as_u64().unwrap() {
            1 => true,
            0 => false,
            _ => unreachable!(),
        };
        let openid = &res["openid"];
        let openid = openid.as_string().unwrap();
        let nickname = &res["nickname"];
        let nickname = nickname.as_string().unwrap();
        let sex = &res["sex"];
        let sex = sex.as_u64().unwrap();
        let language = &res["language"];
        let language = language.as_string().unwrap();
        let city = &res["city"];
        let city = city.as_string().unwrap();
        let province = &res["province"];
        let province = province.as_string().unwrap();
        let country = &res["country"];
        let country = country.as_string().unwrap();
        let avatar = &res["headimgurl"];
        let avatar = avatar.as_string().unwrap();
        let subscribe_time = &res["subscribe_time"];
        let subscribe_time = subscribe_time.as_u64().unwrap();
        let unionid = match res.find("unionid") {
            Some(ref uid) => {
                let _uid = uid.as_string().unwrap();
                Some(_uid.to_owned())
            },
            None => None,
        };
        let remark = &res["remark"];
        let remark = remark.as_string().unwrap();
        let group_id = &res["groupid"];
        let group_id = group_id.as_u64().unwrap();
        User {
            subscribe: subscribe,
            openid: openid.to_owned(),
            nickname: nickname.to_owned(),
            sex: sex as u8,
            language: language.to_owned(),
            city: city.to_owned(),
            province: province.to_owned(),
            country: country.to_owned(),
            avatar: avatar.to_owned(),
            subscribe_time: subscribe_time,
            unionid: unionid,
            remark: remark.to_owned(),
            group_id: group_id,
        }
    }

    pub fn get_batch(&self, user_list: &[HashMap<String, String>]) -> WeChatResult<Vec<User>> {
        let data = jsonway::object(|obj| {
            obj.set("user_list", user_list.to_vec());
        }).unwrap();
        let res = try!(self.client.post("user/info/batchget", vec![], &data));
        let info_list = &res["user_info_list"];
        let info_list = info_list.as_array().unwrap();
        let mut users = vec![];
        for info in info_list {
            users.push(self.json_to_user(&info));
        }
        Ok(users)
    }

    pub fn get_batch_with_lang(&self, user_list: &[String], lang: &str) -> WeChatResult<Vec<User>> {
        let mut users = vec![];
        for openid in user_list {
            let mut user = HashMap::new();
            user.insert("openid".to_owned(), openid.to_owned());
            user.insert("lang".to_owned(), lang.to_owned());
            users.push(user);
        }
        self.get_batch(&users)
    }
}
