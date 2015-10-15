use rustc_serialize::json::Json;

use client::WeChatClient;
use errors::WeChatError;

#[derive(Debug, Clone)]
pub struct WeChatGroup<'a> {
    client: &'a WeChatClient,
}

impl<'a> WeChatGroup<'a> {

    #[inline]
    pub fn new(client: &'a WeChatClient) -> WeChatGroup<'a> {
        WeChatGroup {
            client: client,
        }
    }

    pub fn create(&self, name: &str) -> Result<Json, WeChatError> {
        let data = json!({
            "group": {
                "name": (name)
            }
        });
        self.client.post("groups/create", vec![], &data)
    }

    pub fn list(&self) -> Result<Vec<Json>, WeChatError> {
        let res = try!(self.client.get("groups/get", vec![]));
        let groups = res.find("groups").unwrap();
        let groups_array = groups.as_array().unwrap();
        Ok(groups_array.to_vec())
    }

    pub fn update(&self, group_id: u64, name: &str) -> Result<Json, WeChatError> {
        let data = json!({
            "group": {
                "id": (group_id),
                "name": (name)
            }
        });
        self.client.post("groups/update", vec![], &data)
    }

    pub fn delete(&self, group_id: u64) -> Result<Json, WeChatError> {
        let data = json!({
            "group": {
                "id": (group_id)
            }
        });
        self.client.post("groups/delete", vec![], &data)
    }

    pub fn move_user(&self, openid: &str, group_id: u64) -> Result<Json, WeChatError> {
        let data = json!({
            "openid": (openid),
            "to_groupid": (group_id)
        });
        self.client.post("groups/members/update", vec![], &data)
    }

    pub fn move_users(&self, openids: Vec<String>, group_id: u64) -> Result<Json, WeChatError> {
        let data = json!({
            "openid_list": (openids),
            "to_groupid": (group_id)
        });
        self.client.post("groups/members/batchupdate", vec![], &data)
    }
}